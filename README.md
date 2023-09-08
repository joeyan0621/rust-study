## homework03

冒泡排序 sort.rs

```rust
fn sort() {
    let mut arr_i32 = [45, 23, 78, 22, 3, 14, 12];
    let mut arr_str = [
        "ruby", "java", "BASIC", "nest", "Lisp", "rust", "Perl", "Erlang",
    ];
    println!("待排序的数组为：{:?}", copy_slice(&arr_i32));
    bubble_sort(&mut arr_i32);
    println!("排序后的数组为：{:?}", arr_i32);

    println!("待排序的数组为：{:?}", copy_slice(&arr_str));
    bubble_sort_tmp(&mut arr_str);
    println!("排序后的数组为：{:?}", arr_str);
}
fn bubble_sort(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn copy_slice<T: Clone>(arr: &[T]) -> Vec<T> {
    arr.to_vec()
}

fn bubble_sort_tmp<T: PartialOrd>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

```

## homework04

红绿灯 light.rs

```rust
// 1. 为枚举交通信号灯实现一个trait，trait中包含一个返回时间的方法，不同的灯持续的时间不同
enum TrafficLight {
    Red,
    Green,
    Yellow,
}
trait TrafficLightDuration {
    fn duration(&self) -> u32;
}
impl TrafficLightDuration for TrafficLight {
    fn duration(&self) -> u32 {
        match self {
            TrafficLight::Green => 50,
            TrafficLight::Red => 20,
            TrafficLight::Yellow => 5,
        }
    }
}

fn traffic_light() {
    let red = TrafficLight::Red;
    let green = TrafficLight::Green;
    let yellow = TrafficLight::Yellow;
    println!("🚥 Red light duration: {} seconds", red.duration());
    println!("🚥 Green light duration: {} seconds", green.duration());
    println!("🚥 Yellow light duration: {} seconds", yellow.duration());
}

```

求和 sum.rs

```rust
// 2. 实现一个函数，为u32类型的整数集合求和，参数类型为&[u32]，返回类型为Option，溢出时返回None
fn sum_u32s(numbers: &[u32]) -> Option<u32> {
    let mut result: Option<u32> = Some(0); // 初始结果为0
    for &num in numbers {
        match result {
            Some(current) => {
                // 使用checked_add方法进行加法并检查溢出
                match current.checked_add(num) {
                    Some(sum) => {
                        result = Some(sum);
                    }
                    None => {
                        // 溢出时返回None
                        result = None;
                        break;
                    }
                }
            }
            None => {
                // 如果之前已经溢出，不再执行加法
                break;
            }
        }
    }
    result
}

fn sum() {
    let numbers = [1, 2, 3, 4, 5];
    let result = sum_u32s(&numbers);

    match result {
        Some(sum) => {
            println!("Sum: {}", sum);
        }
        None => {
            println!("Overflow occurred.");
        }
    }
}

```

计算面积 area.rs

```rust
// 3. 实现一个打印图形面积的函数，接受一个可以计算面积的类型作为参数，比如圆形、三角形、正方形，需要使用到泛型和泛型约束
trait Area {
    fn area(&self) -> f64;
}
struct Circle {
    radius: f64,
}
struct Rect {
    width: f64,
    height: f64,
}
impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}
impl Area for Rect {
    fn area(&self) -> f64 {
        self.height * self.width
    }
}
fn print_area<T: Area>(shape: T) {
    let area = shape.area();
    println!("The area is: {}", area);
}
fn area() {
    let circle = Circle { radius: 5.0 };
    let triangle = Rect {
        width: 4.0,
        height: 7.0,
    };
    print_area(circle);
    print_area(triangle);
}

```

## homework05

```rust
#![cfg_attr(not(feature = "std"), no_std)]
/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		#[pallet::constant]
		type MaxClaimLength: Get<u32>;
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn proofs)]
	pub type Proofs<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BoundedVec<u8, T::MaxClaimLength>,
		(T::AccountId, T::BlockNumber),
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ClaimCreated(T::AccountId, BoundedVec<u8, T::MaxClaimLength>),
		ClaimRevoked(T::AccountId, BoundedVec<u8, T::MaxClaimLength>),
		ClaimTransferred(T::AccountId, T::AccountId, BoundedVec<u8, T::MaxClaimLength>),
	}

	#[pallet::error]
	pub enum Error<T> {
		ProofAlreadyExist,
		ClaimTooLong,
		ClaimNotExist,
		NotClaimOwner,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)] // 可调用顺序
		#[pallet::weight(0)] // 指定权重
		pub fn create_claim(
			origin: OriginFor<T>,
			claim: BoundedVec<u8, T::MaxClaimLength>,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofAlreadyExist);

			Proofs::<T>::insert(
				&claim,
				(sender.clone(), frame_system::Pallet::<T>::block_number()),
			);
			// 存储成功后触发一个ClaimCreated事件，存证被创建
			Self::deposit_event(Event::ClaimCreated(sender, claim));
			Ok(().into())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(0)]
		pub fn revoke_claim(
			origin: OriginFor<T>,
			claim: BoundedVec<u8, T::MaxClaimLength>,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			let (owner, _) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?;
			ensure!(owner == sender, Error::<T>::NotClaimOwner);

			Proofs::<T>::remove(&claim);

			Self::deposit_event(Event::ClaimRevoked(sender, claim));
			Ok(().into())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(0)]
		pub fn transfer_claim(
			origin: OriginFor<T>,
			claim: BoundedVec<u8, T::MaxClaimLength>,
			to: T::AccountId,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			let (owner, block_number) =
				Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?;

			ensure!(owner == sender, Error::<T>::NotClaimOwner);

			Proofs::<T>::insert(&claim, (to.clone(), block_number));
			Self::deposit_event(Event::ClaimTransferred(sender, to, claim));
			Ok(().into())
		}
	}
}

```
