use nalgebra as na;
use opencv::{
    core::{Point2f, Rect},
    prelude::*,
};
use ownref::ArcRefA as ARef;
use r2r::{
    sensor_msgs::msg::{Image, PointCloud2},
    vision_msgs::msg::Detection2DArray,
};

pub type ArcPointVec = ARef<'static, Vec<Point>>;
pub type ArcPoint = ARef<'static, Vec<Point>, Point>;
pub type ArcRectVec = ARef<'static, Vec<Rect>>;
pub type ArcRect = ARef<'static, Vec<Rect>, Rect>;
pub type ArcAssocVec = ARef<'static, Vec<Association>>;

#[derive(Debug)]
pub enum InputMessage {
    PointCloud2(PointCloud2),
    OtobriteImage(Image),
    BBox(Detection2DArray),
}

#[derive(Debug)]
pub enum FuseMessage {
    Otobrite(OtobriteMessage),
    Kneron(KneronMessage),
    Kiss3d(Kiss3dMessage),
}

impl From<Kiss3dMessage> for FuseMessage {
    fn from(v: Kiss3dMessage) -> Self {
        Self::Kiss3d(v)
    }
}

impl From<KneronMessage> for FuseMessage {
    fn from(v: KneronMessage) -> Self {
        Self::Kneron(v)
    }
}

impl From<OtobriteMessage> for FuseMessage {
    fn from(v: OtobriteMessage) -> Self {
        Self::Otobrite(v)
    }
}

#[derive(Debug)]
pub struct OtobriteMessage {
    pub image: Option<Mat>,
    pub assocs: Option<ArcAssocVec>,
}

#[derive(Debug)]
pub struct KneronMessage {
    pub rects: Option<ArcRectVec>,
    pub assocs: Option<ArcAssocVec>,
}

#[derive(Debug)]
pub struct Kiss3dMessage {
    pub points: ArcPointVec,
    pub kneron_assocs: Option<ArcAssocVec>,
}

#[derive(Debug)]
pub enum OpencvMessage {
    Otobrite(OtobriteMessage),
    Kneron(KneronMessage),
}

impl From<KneronMessage> for OpencvMessage {
    fn from(v: KneronMessage) -> Self {
        Self::Kneron(v)
    }
}

impl From<OtobriteMessage> for OpencvMessage {
    fn from(v: OtobriteMessage) -> Self {
        Self::Otobrite(v)
    }
}

#[derive(Debug)]
pub struct Point {
    pub position: na::Point3<f32>,
    pub intensity: f32,
}

#[derive(Debug)]
pub struct Association {
    pub pcd_point: ArcPoint,
    pub img_point: Point2f,
    pub rect: Option<ArcRect>,
}
