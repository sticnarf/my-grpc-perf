package main

import (
	"context"
	"fmt"
	"grpc-demo/tikvpb"
	"sync/atomic"
	"time"

	"google.golang.org/grpc"
)

func main() {
	conn, err := grpc.Dial("[::1]:50051", grpc.WithInsecure(), grpc.WithBlock())
	if err != nil {
		panic(err)
	}
	defer conn.Close()
	client := tikvpb.NewTikvClient(conn)

	count := uint64(0)
	for i := 0; i < 16; i++ {
		go func() {
			for {
				resp, err := client.KvGet(context.Background(), &tikvpb.GetRequest{Key: []byte("key")})
				if err != nil {
					panic(err)
				}
				if string(resp.Value) != "key" {
					panic("unexpected response")
				}
				atomic.AddUint64(&count, 1)
			}
		}()
	}
	last := uint64(0)
	ticker := time.Tick(time.Second)
	for range ticker {
		current := atomic.LoadUint64(&count)
		fmt.Println(current - last)
		last = current
	}
}
