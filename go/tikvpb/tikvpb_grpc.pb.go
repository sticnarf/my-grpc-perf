// Code generated by protoc-gen-go-grpc. DO NOT EDIT.

package tikvpb

import (
	context "context"
	grpc "google.golang.org/grpc"
	codes "google.golang.org/grpc/codes"
	status "google.golang.org/grpc/status"
)

// This is a compile-time assertion to ensure that this generated file
// is compatible with the grpc package it is being compiled against.
// Requires gRPC-Go v1.32.0 or later.
const _ = grpc.SupportPackageIsVersion7

// TikvClient is the client API for Tikv service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://pkg.go.dev/google.golang.org/grpc/?tab=doc#ClientConn.NewStream.
type TikvClient interface {
	// Commands using a transactional interface.
	KvGet(ctx context.Context, in *GetRequest, opts ...grpc.CallOption) (*GetResponse, error)
}

type tikvClient struct {
	cc grpc.ClientConnInterface
}

func NewTikvClient(cc grpc.ClientConnInterface) TikvClient {
	return &tikvClient{cc}
}

func (c *tikvClient) KvGet(ctx context.Context, in *GetRequest, opts ...grpc.CallOption) (*GetResponse, error) {
	out := new(GetResponse)
	err := c.cc.Invoke(ctx, "/tikvpb.Tikv/KvGet", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

// TikvServer is the server API for Tikv service.
// All implementations must embed UnimplementedTikvServer
// for forward compatibility
type TikvServer interface {
	// Commands using a transactional interface.
	KvGet(context.Context, *GetRequest) (*GetResponse, error)
	mustEmbedUnimplementedTikvServer()
}

// UnimplementedTikvServer must be embedded to have forward compatible implementations.
type UnimplementedTikvServer struct {
}

func (UnimplementedTikvServer) KvGet(context.Context, *GetRequest) (*GetResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method KvGet not implemented")
}
func (UnimplementedTikvServer) mustEmbedUnimplementedTikvServer() {}

// UnsafeTikvServer may be embedded to opt out of forward compatibility for this service.
// Use of this interface is not recommended, as added methods to TikvServer will
// result in compilation errors.
type UnsafeTikvServer interface {
	mustEmbedUnimplementedTikvServer()
}

func RegisterTikvServer(s grpc.ServiceRegistrar, srv TikvServer) {
	s.RegisterService(&Tikv_ServiceDesc, srv)
}

func _Tikv_KvGet_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(GetRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(TikvServer).KvGet(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/tikvpb.Tikv/KvGet",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(TikvServer).KvGet(ctx, req.(*GetRequest))
	}
	return interceptor(ctx, in, info, handler)
}

// Tikv_ServiceDesc is the grpc.ServiceDesc for Tikv service.
// It's only intended for direct use with grpc.RegisterService,
// and not to be introspected or modified (even as a copy)
var Tikv_ServiceDesc = grpc.ServiceDesc{
	ServiceName: "tikvpb.Tikv",
	HandlerType: (*TikvServer)(nil),
	Methods: []grpc.MethodDesc{
		{
			MethodName: "KvGet",
			Handler:    _Tikv_KvGet_Handler,
		},
	},
	Streams:  []grpc.StreamDesc{},
	Metadata: "tikvpb.proto",
}