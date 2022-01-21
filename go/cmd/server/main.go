package main

import (
	"context"
	"grpc-demo/tikvpb"
	"net"

	"google.golang.org/grpc"
)

type tikvImpl struct {
	tikvpb.UnimplementedTikvServer
}

func (*tikvImpl) KvGet(_ context.Context, req *tikvpb.GetRequest) (*tikvpb.GetResponse, error) {
	return &tikvpb.GetResponse{Value: req.Key}, nil
}

func main() {
	lis, err := net.Listen("tcp", "[::1]:50051")
	if err != nil {
		panic(err)
	}
	s := grpc.NewServer()
	tikvpb.RegisterTikvServer(s, &tikvImpl{})
	if err := s.Serve(lis); err != nil {
		panic(err)
	}
}
