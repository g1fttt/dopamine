#pragma once

#include <cstddef>

template <typename T> struct Ptr {
  constexpr Ptr(T *value) : value{value} {}
  constexpr Ptr(size_t address) : value{reinterpret_cast<T *>(address)} {}
  constexpr Ptr() : value{nullptr} {}

  constexpr void operator=(T *other) {
    value = other;
  }

  template <typename U = T> constexpr const U &operator*() const {
    return *value;
  }

  template <typename U = T> constexpr U &operator*() {
    return *value;
  }

  template <typename U = T> constexpr U *operator->() const {
    return value;
  }

  template <typename U = T> constexpr U *operator->() {
    return value;
  }

  constexpr T *get() {
    return value;
  }

  constexpr const T *get() const {
    return value;
  }

  template <typename U> constexpr Ptr<U> cast() const {
    return reinterpret_cast<U *>(value);
  }

  constexpr Ptr<T> add(size_t offset) const {
    return value + offset;
  }

  constexpr Ptr<T> sub(size_t offset) const {
    return value - offset;
  }

  constexpr Ptr<T> byte_add(size_t offset) const {
    return cast<std::byte>().add(offset).template cast<T>();
  }

  constexpr Ptr<T> byte_sub(size_t offset) const {
    return cast<std::byte>().sub(offset).template cast<T>();
  }
private:
  T *value;
};
