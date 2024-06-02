pub trait Image {
    type Pixel;

    fn height(&self) -> usize;
    fn width(&self) -> usize;

    fn pixel(&self, y : usize, x : usize) -> &Self::Pixel;
    fn pixel_mut(&mut self, y : usize, x : usize) -> &mut Self::Pixel;

    fn pixels(&self) -> impl Iterator<Item = &Self::Pixel>;
    fn pixels_mut(&mut self) -> impl Iterator<Item = &mut Self::Pixel>;
}

impl<P, Container> Image for image::ImageBuffer<P, Container>
where
    P: image::Pixel,
    Container: std::ops::Deref<Target = [P::Subpixel]> + std::ops::DerefMut
{
    type Pixel = P;

    fn height(&self) -> usize {
        self.height() as usize
    }

    fn width(&self) -> usize {
        self.width() as usize
    }

    fn pixel(&self, y : usize, x : usize) -> &Self::Pixel {
        self.get_pixel(x as u32, y as u32)
    }

    fn pixel_mut(&mut self, y : usize, x : usize) -> &mut Self::Pixel {
        self.get_pixel_mut(x as u32, y as u32)
    }

    fn pixels(&self) -> impl Iterator<Item = &Self::Pixel> {
        self.pixels()
    }

    fn pixels_mut(&mut self) -> impl Iterator<Item = &mut Self::Pixel> {
        self.pixels_mut()
    }
}

