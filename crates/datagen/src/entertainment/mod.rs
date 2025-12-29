//! Entertainment data generators (books, movies, music, TV, games).
//!
//! Provides generators for entertainment-related fake data including
//! book titles, movie information, music artists, TV shows, and video games.

pub mod books;
pub mod games;
pub mod movies;
pub mod music;
pub mod tv;

// Re-export common functions
pub use books::{book_author, book_genre, book_publisher, book_series, book_title};
pub use games::{game_genre, game_platform, game_studio, game_title};
pub use movies::{movie_actor, movie_director, movie_genre, movie_rating, movie_title};
pub use music::{music_album, music_artist, music_genre, music_instrument, music_song};
pub use tv::{tv_channel, tv_genre, tv_network, tv_show};
