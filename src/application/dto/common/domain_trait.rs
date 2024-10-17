pub trait ToProtoTrait {
    type ProtoType;
    fn to_proto(self) -> Self::ProtoType;
}
