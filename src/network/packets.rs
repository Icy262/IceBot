use crate::network::data_types::*;
use std::io::{Read, Write};

pub(crate) enum Packets {
	KeepAlive(KeepAlive),
	LoginRequest(LoginRequest),
	Handshake(Handshake),
	ChatMessage(ChatMessage),
	TimeUpdate(TimeUpdate),
	EntityEquipment(EntityEquipment),
	SpawnPosition(SpawnPosition),
	UseEntity(UseEntity),
	UpdateHealth(UpdateHealth),
	Respawn(Respawn),
	Player(Player),
	PlayerPosition(PlayerPosition),
	PlayerLook(PlayerLook),
	PlayerPositionandLook(PlayerPositionandLook),
	PlayerDigging(PlayerDigging),
	PlayerBlockPlacement(PlayerBlockPlacement),
	HoldingChange(HoldingChange),
	UseBed(UseBed),
	Animation(Animation),
	EntityAction(EntityAction),
	NamedEntitySpawn(NamedEntitySpawn),
	PickupSpawn(PickupSpawn),
	CollectItem(CollectItem),
	AddObjectorVehicle(AddObjectorVehicle),
	MobSpawn(MobSpawn),
	Painting(Painting),
	StanceUpdate(StanceUpdate),
	EntityVelocity(EntityVelocity),
	DestroyEntity(DestroyEntity),
	Entity(Entity),
	EntityRelativeMove(EntityRelativeMove),
	EntityLook(EntityLook),
	EntityLookandRelativeMove(EntityLookandRelativeMove),
	EntityTeleport(EntityTeleport),
	EntityStatus(EntityStatus),
	AttachEntity(AttachEntity),
	Entitymetadata(Entitymetadata),
	PreChunk(PreChunk),
	MapChunk(MapChunk),
	MultiBlockChange(MultiBlockChange),
	BlockChange(BlockChange),
	BlockAction(BlockAction),
	Explosion(Explosion),
	SoundEffect(SoundEffect),
	NeworInvalidState(NeworInvalidState),
	Thunderbolt(Thunderbolt),
	OpenWindow(OpenWindow),
	CloseWindow(CloseWindow),
	WindowClick(WindowClick),
	SetSlot(SetSlot),
	WindowItems(WindowItems),
	UpdateProgressBar(UpdateProgressBar),
	Transaction(Transaction),
	UpdateSign(UpdateSign),
	ItemData(ItemData),
	IncrementStatistic(IncrementStatistic),
	DisconnectorKick(DisconnectorKick),
}

pub(crate) fn read_packet<R: Read>(stream: &mut R) -> Option<Packets> {
	let id = MCUByte::read(stream).value;

	let packet = match id {
		KeepAlive::ID => Packets::KeepAlive(KeepAlive::read(stream)),
		LoginRequest::ID => Packets::LoginRequest(LoginRequest::read(stream)),
		Handshake::ID => Packets::Handshake(Handshake::read(stream)),
		ChatMessage::ID => Packets::ChatMessage(ChatMessage::read(stream)),
		TimeUpdate::ID => Packets::TimeUpdate(TimeUpdate::read(stream)),
		EntityEquipment::ID => Packets::EntityEquipment(EntityEquipment::read(stream)),
		SpawnPosition::ID => Packets::SpawnPosition(SpawnPosition::read(stream)),
		UseEntity::ID => Packets::UseEntity(UseEntity::read(stream)),
		UpdateHealth::ID => Packets::UpdateHealth(UpdateHealth::read(stream)),
		Respawn::ID => Packets::Respawn(Respawn::read(stream)),
		Player::ID => Packets::Player(Player::read(stream)),
		PlayerPosition::ID => Packets::PlayerPosition(PlayerPosition::read(stream)),
		PlayerLook::ID => Packets::PlayerLook(PlayerLook::read(stream)),
		PlayerPositionandLook::ID => {
			Packets::PlayerPositionandLook(PlayerPositionandLook::read(stream))
		}
		PlayerDigging::ID => Packets::PlayerDigging(PlayerDigging::read(stream)),
		PlayerBlockPlacement::ID => {
			Packets::PlayerBlockPlacement(PlayerBlockPlacement::read(stream))
		}
		HoldingChange::ID => Packets::HoldingChange(HoldingChange::read(stream)),
		UseBed::ID => Packets::UseBed(UseBed::read(stream)),
		Animation::ID => Packets::Animation(Animation::read(stream)),
		EntityAction::ID => Packets::EntityAction(EntityAction::read(stream)),
		NamedEntitySpawn::ID => Packets::NamedEntitySpawn(NamedEntitySpawn::read(stream)),
		PickupSpawn::ID => Packets::PickupSpawn(PickupSpawn::read(stream)),
		CollectItem::ID => Packets::CollectItem(CollectItem::read(stream)),
		AddObjectorVehicle::ID => Packets::AddObjectorVehicle(AddObjectorVehicle::read(stream)),
		MobSpawn::ID => Packets::MobSpawn(MobSpawn::read(stream)),
		Painting::ID => Packets::Painting(Painting::read(stream)),
		StanceUpdate::ID => Packets::StanceUpdate(StanceUpdate::read(stream)),
		EntityVelocity::ID => Packets::EntityVelocity(EntityVelocity::read(stream)),
		DestroyEntity::ID => Packets::DestroyEntity(DestroyEntity::read(stream)),
		Entity::ID => Packets::Entity(Entity::read(stream)),
		EntityRelativeMove::ID => Packets::EntityRelativeMove(EntityRelativeMove::read(stream)),
		EntityLook::ID => Packets::EntityLook(EntityLook::read(stream)),
		EntityLookandRelativeMove::ID => {
			Packets::EntityLookandRelativeMove(EntityLookandRelativeMove::read(stream))
		}
		EntityTeleport::ID => Packets::EntityTeleport(EntityTeleport::read(stream)),
		EntityStatus::ID => Packets::EntityStatus(EntityStatus::read(stream)),
		AttachEntity::ID => Packets::AttachEntity(AttachEntity::read(stream)),
		Entitymetadata::ID => Packets::Entitymetadata(Entitymetadata::read(stream)),
		PreChunk::ID => Packets::PreChunk(PreChunk::read(stream)),
		MapChunk::ID => Packets::MapChunk(MapChunk::read(stream)),
		MultiBlockChange::ID => Packets::MultiBlockChange(MultiBlockChange::read(stream)),
		BlockChange::ID => Packets::BlockChange(BlockChange::read(stream)),
		BlockAction::ID => Packets::BlockAction(BlockAction::read(stream)),
		Explosion::ID => Packets::Explosion(Explosion::read(stream)),
		SoundEffect::ID => Packets::SoundEffect(SoundEffect::read(stream)),
		NeworInvalidState::ID => Packets::NeworInvalidState(NeworInvalidState::read(stream)),
		Thunderbolt::ID => Packets::Thunderbolt(Thunderbolt::read(stream)),
		OpenWindow::ID => Packets::OpenWindow(OpenWindow::read(stream)),
		CloseWindow::ID => Packets::CloseWindow(CloseWindow::read(stream)),
		WindowClick::ID => Packets::WindowClick(WindowClick::read(stream)),
		SetSlot::ID => Packets::SetSlot(SetSlot::read(stream)),
		WindowItems::ID => Packets::WindowItems(WindowItems::read(stream)),
		UpdateProgressBar::ID => Packets::UpdateProgressBar(UpdateProgressBar::read(stream)),
		Transaction::ID => Packets::Transaction(Transaction::read(stream)),
		UpdateSign::ID => Packets::UpdateSign(UpdateSign::read(stream)),
		ItemData::ID => Packets::ItemData(ItemData::read(stream)),
		IncrementStatistic::ID => Packets::IncrementStatistic(IncrementStatistic::read(stream)),
		DisconnectorKick::ID => Packets::DisconnectorKick(DisconnectorKick::read(stream)),
		_ => return None,
	};

	return Some(packet);
}

pub(crate) fn write_packet<W: Write>(stream: &mut W, packet: Packets) {
	match packet {
		Packets::KeepAlive(packet) => KeepAlive::write(stream, packet),
		Packets::LoginRequest(packet) => LoginRequest::write(stream, packet),
		Packets::Handshake(packet) => Handshake::write(stream, packet),
		Packets::ChatMessage(packet) => ChatMessage::write(stream, packet),
		Packets::TimeUpdate(packet) => TimeUpdate::write(stream, packet),
		Packets::EntityEquipment(packet) => EntityEquipment::write(stream, packet),
		Packets::SpawnPosition(packet) => SpawnPosition::write(stream, packet),
		Packets::UseEntity(packet) => UseEntity::write(stream, packet),
		Packets::UpdateHealth(packet) => UpdateHealth::write(stream, packet),
		Packets::Respawn(packet) => Respawn::write(stream, packet),
		Packets::Player(packet) => Player::write(stream, packet),
		Packets::PlayerPosition(packet) => PlayerPosition::write(stream, packet),
		Packets::PlayerLook(packet) => PlayerLook::write(stream, packet),
		Packets::PlayerPositionandLook(packet) => PlayerPositionandLook::write(stream, packet),
		Packets::PlayerDigging(packet) => PlayerDigging::write(stream, packet),
		Packets::PlayerBlockPlacement(packet) => PlayerBlockPlacement::write(stream, packet),
		Packets::HoldingChange(packet) => HoldingChange::write(stream, packet),
		Packets::UseBed(packet) => UseBed::write(stream, packet),
		Packets::Animation(packet) => Animation::write(stream, packet),
		Packets::EntityAction(packet) => EntityAction::write(stream, packet),
		Packets::NamedEntitySpawn(packet) => NamedEntitySpawn::write(stream, packet),
		Packets::PickupSpawn(packet) => PickupSpawn::write(stream, packet),
		Packets::CollectItem(packet) => CollectItem::write(stream, packet),
		Packets::AddObjectorVehicle(packet) => AddObjectorVehicle::write(stream, packet),
		Packets::MobSpawn(packet) => MobSpawn::write(stream, packet),
		Packets::Painting(packet) => Painting::write(stream, packet),
		Packets::StanceUpdate(packet) => StanceUpdate::write(stream, packet),
		Packets::EntityVelocity(packet) => EntityVelocity::write(stream, packet),
		Packets::DestroyEntity(packet) => DestroyEntity::write(stream, packet),
		Packets::Entity(packet) => Entity::write(stream, packet),
		Packets::EntityRelativeMove(packet) => EntityRelativeMove::write(stream, packet),
		Packets::EntityLook(packet) => EntityLook::write(stream, packet),
		Packets::EntityLookandRelativeMove(packet) => {
			EntityLookandRelativeMove::write(stream, packet)
		}
		Packets::EntityTeleport(packet) => EntityTeleport::write(stream, packet),
		Packets::EntityStatus(packet) => EntityStatus::write(stream, packet),
		Packets::AttachEntity(packet) => AttachEntity::write(stream, packet),
		Packets::Entitymetadata(packet) => Entitymetadata::write(stream, packet),
		Packets::PreChunk(packet) => PreChunk::write(stream, packet),
		Packets::MapChunk(packet) => MapChunk::write(stream, packet),
		Packets::MultiBlockChange(packet) => MultiBlockChange::write(stream, packet),
		Packets::BlockChange(packet) => BlockChange::write(stream, packet),
		Packets::BlockAction(packet) => BlockAction::write(stream, packet),
		Packets::Explosion(packet) => Explosion::write(stream, packet),
		Packets::SoundEffect(packet) => SoundEffect::write(stream, packet),
		Packets::NeworInvalidState(packet) => NeworInvalidState::write(stream, packet),
		Packets::Thunderbolt(packet) => Thunderbolt::write(stream, packet),
		Packets::OpenWindow(packet) => OpenWindow::write(stream, packet),
		Packets::CloseWindow(packet) => CloseWindow::write(stream, packet),
		Packets::WindowClick(packet) => WindowClick::write(stream, packet),
		Packets::SetSlot(packet) => SetSlot::write(stream, packet),
		Packets::WindowItems(packet) => WindowItems::write(stream, packet),
		Packets::UpdateProgressBar(packet) => UpdateProgressBar::write(stream, packet),
		Packets::Transaction(packet) => Transaction::write(stream, packet),
		Packets::UpdateSign(packet) => UpdateSign::write(stream, packet),
		Packets::ItemData(packet) => ItemData::write(stream, packet),
		Packets::IncrementStatistic(packet) => IncrementStatistic::write(stream, packet),
		Packets::DisconnectorKick(packet) => DisconnectorKick::write(stream, packet),
	};
}

pub(crate) struct KeepAlive {}

impl KeepAlive {
	const ID: u8 = 0;
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
		Self {}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
		MCUByte::write(stream, MCUByte { value: Self::ID });
	}
}

pub(crate) struct LoginRequest {
	pub(crate) protocol_version: MCInt,
	pub(crate) username: MCString16,
	pub(crate) map_seed: MCLong,
	pub(crate) dimension: MCByte,
}

impl LoginRequest {
	const ID: u8 = 1;
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
		Self {
			protocol_version: MCInt::read(stream),
			username: MCString16::read(stream),
			map_seed: MCLong::read(stream),
			dimension: MCByte::read(stream),
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
		MCUByte::write(stream, MCUByte { value: Self::ID });
		MCInt::write(stream, data.protocol_version);
		MCString16::write(stream, data.username);
		MCLong::write(stream, data.map_seed);
		MCByte::write(stream, data.dimension);
	}
}

pub(crate) struct Handshake {
	pub(crate) username: MCString16,
}

impl Handshake {
	const ID: u8 = 2;
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
		Self {
			username: MCString16::read(stream),
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
		MCUByte::write(stream, MCUByte { value: Self::ID });
		MCString16::write(stream, data.username);
	}
}

pub(crate) struct ChatMessage {
	pub(crate) message: MCString16,
}

impl ChatMessage {
	const ID: u8 = 3;
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
		Self {
			message: MCString16::read(stream),
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
		MCUByte::write(stream, MCUByte { value: Self::ID });
		MCString16::write(stream, data.message);
	}
}

pub(crate) struct TimeUpdate {
	pub(crate) time: MCLong,
}

impl TimeUpdate {
	const ID: u8 = 4;
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
		Self {
			time: MCLong::read(stream),
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
		MCUByte::write(stream, MCUByte { value: Self::ID });
		MCLong::write(stream, data.time);
	}
}

pub(crate) struct EntityEquipment {
	pub(crate) entity_id: MCInt,
	pub(crate) slot: MCShort,
	pub(crate) item_id: MCShort,
	pub(crate) unknown: MCShort,
}

impl EntityEquipment {
	const ID: u8 = 5;
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
		Self {
			entity_id: MCInt::read(stream),
			slot: MCShort::read(stream),
			item_id: MCShort::read(stream),
			unknown: MCShort::read(stream),
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
		MCUByte::write(stream, MCUByte { value: Self::ID });
		MCInt::write(stream, data.entity_id);
		MCShort::write(stream, data.slot);
		MCShort::write(stream, data.item_id);
		MCShort::write(stream, data.unknown);
	}
}

pub(crate) struct SpawnPosition {
	pub(crate) x: MCInt,
	pub(crate) y: MCInt,
	pub(crate) z: MCInt,
}

impl SpawnPosition {
	const ID: u8 = 6;
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
		Self {
			x: MCInt::read(stream),
			y: MCInt::read(stream),
			z: MCInt::read(stream),
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
		MCUByte::write(stream, MCUByte { value: Self::ID });
		MCInt::write(stream, data.x);
		MCInt::write(stream, data.y);
		MCInt::write(stream, data.z);
	}
}

pub(crate) struct UseEntity {
	pub(crate) user: MCInt,
	pub(crate) target: MCInt,
	pub(crate) left_click: MCBool,
}

impl UseEntity {
	const ID: u8 = 7;
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
		Self {
			user: MCInt::read(stream),
			target: MCInt::read(stream),
			left_click: MCBool::read(stream),
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
		MCUByte::write(stream, MCUByte { value: Self::ID });
		MCInt::write(stream, data.user);
		MCInt::write(stream, data.target);
		MCBool::write(stream, data.left_click);
	}
}

pub(crate) struct UpdateHealth {
	pub(crate) health: MCShort,
}

impl UpdateHealth {
	const ID: u8 = 8;
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
		Self {
			health: MCShort::read(stream),
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
		MCUByte::write(stream, MCUByte { value: Self::ID });
		MCShort::write(stream, data.health);
	}
}

pub(crate) struct Respawn {
	pub(crate) world: MCByte,
}

impl Respawn {
	const ID: u8 = 9;
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
		Self {
			world: MCByte::read(stream),
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
		MCUByte::write(stream, MCUByte { value: Self::ID });
		MCByte::write(stream, data.world);
	}
}

pub(crate) struct Player {
	pub(crate) on_ground: MCBool,
}

impl Player {
	const ID: u8 = 10;
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
		Self {
			on_ground: MCBool::read(stream),
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
		MCUByte::write(stream, MCUByte { value: Self::ID });
		MCBool::write(stream, data.on_ground);
	}
}

pub(crate) struct PlayerPosition {
	pub(crate) x: MCDouble,
	pub(crate) y: MCDouble,
	pub(crate) stance: MCDouble,
	pub(crate) z: MCDouble,
	pub(crate) on_ground: MCBool,
}

impl PlayerPosition {
	const ID: u8 = 11;
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
		Self {
			x: MCDouble::read(stream),
			y: MCDouble::read(stream),
			stance: MCDouble::read(stream),
			z: MCDouble::read(stream),
			on_ground: MCBool::read(stream),
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
		MCUByte::write(stream, MCUByte { value: Self::ID });
		MCDouble::write(stream, data.x);
		MCDouble::write(stream, data.y);
		MCDouble::write(stream, data.stance);
		MCDouble::write(stream, data.z);
		MCBool::write(stream, data.on_ground);
	}
}

pub(crate) struct PlayerLook {
	pub(crate) yaw: MCFloat,
	pub(crate) pitch: MCFloat,
	pub(crate) on_ground: MCBool,
}

impl PlayerLook {
	const ID: u8 = 12;
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
		Self {
			yaw: MCFloat::read(stream),
			pitch: MCFloat::read(stream),
			on_ground: MCBool::read(stream),
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
		MCUByte::write(stream, MCUByte { value: Self::ID });
		MCFloat::write(stream, data.yaw);
		MCFloat::write(stream, data.pitch);
		MCBool::write(stream, data.on_ground);
	}
}

pub(crate) struct PlayerPositionandLook {
	pub(crate) x: MCDouble,
	pub(crate) y: MCDouble,
	pub(crate) stance: MCDouble,
	pub(crate) z: MCDouble,
	pub(crate) yaw: MCFloat,
	pub(crate) pitch: MCFloat,
	pub(crate) on_ground: MCBool,
}

impl PlayerPositionandLook {
	const ID: u8 = 13;
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
		Self {
			x: MCDouble::read(stream),
			y: MCDouble::read(stream),
			stance: MCDouble::read(stream),
			z: MCDouble::read(stream),
			yaw: MCFloat::read(stream),
			pitch: MCFloat::read(stream),
			on_ground: MCBool::read(stream),
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
		MCUByte::write(stream, MCUByte { value: Self::ID });
		MCDouble::write(stream, data.x);
		MCDouble::write(stream, data.y);
		MCDouble::write(stream, data.stance);
		MCDouble::write(stream, data.z);
		MCFloat::write(stream, data.yaw);
		MCFloat::write(stream, data.pitch);
		MCBool::write(stream, data.on_ground);
	}
}

pub(crate) struct PlayerDigging {
	pub(crate) status: MCByte,
	pub(crate) x: MCInt,
	pub(crate) y: MCByte,
	pub(crate) z: MCInt,
	pub(crate) face: MCByte,
}

impl PlayerDigging {
	const ID: u8 = 14;
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
		Self {
			status: MCByte::read(stream),
			x: MCInt::read(stream),
			y: MCByte::read(stream),
			z: MCInt::read(stream),
			face: MCByte::read(stream),
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
		MCUByte::write(stream, MCUByte { value: Self::ID });
		MCByte::write(stream, data.status);
		MCInt::write(stream, data.x);
		MCByte::write(stream, data.y);
		MCInt::write(stream, data.z);
		MCByte::write(stream, data.face);
	}
}

pub(crate) struct PlayerBlockPlacement {
	pub(crate) x: MCInt,
	pub(crate) y: MCByte,
	pub(crate) z: MCInt,
	pub(crate) direction: MCByte,
	pub(crate) block_or_item_id: MCShort,
	pub(crate) amount: MCByte,
	pub(crate) damage: MCShort,
}

impl PlayerBlockPlacement {
	const ID: u8 = 15;
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
		Self {
			x: MCInt::read(stream),
			y: MCByte::read(stream),
			z: MCInt::read(stream),
			direction: MCByte::read(stream),
			block_or_item_id: MCShort::read(stream),
			amount: MCByte::read(stream),
			damage: MCShort::read(stream),
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
		MCUByte::write(stream, MCUByte { value: Self::ID });
		MCInt::write(stream, data.x);
		MCByte::write(stream, data.y);
		MCInt::write(stream, data.z);
		MCByte::write(stream, data.direction);
		MCShort::write(stream, data.block_or_item_id);
		MCByte::write(stream, data.amount);
		MCShort::write(stream, data.damage);
	}
}

pub(crate) struct HoldingChange {
	pub(crate) slot_id: MCShort,
}

impl HoldingChange {
	const ID: u8 = 16;
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
		Self {
			slot_id: MCShort::read(stream),
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
		MCUByte::write(stream, MCUByte { value: Self::ID });
		MCShort::write(stream, data.slot_id);
	}
}

pub(crate) struct UseBed {
	pub(crate) entity_id: MCInt,
	pub(crate) in_bed: MCByte,
	pub(crate) x: MCInt,
	pub(crate) y: MCByte,
	pub(crate) z: MCInt,
}

impl UseBed {
	const ID: u8 = 17;
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
		Self {
			entity_id: MCInt::read(stream),
			in_bed: MCByte::read(stream),
			x: MCInt::read(stream),
			y: MCByte::read(stream),
			z: MCInt::read(stream),
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
		MCUByte::write(stream, MCUByte { value: Self::ID });
		MCInt::write(stream, data.entity_id);
		MCByte::write(stream, data.in_bed);
		MCInt::write(stream, data.x);
		MCByte::write(stream, data.y);
		MCInt::write(stream, data.z);
	}
}

pub(crate) struct Animation {
	pub(crate) eid: MCInt,
	pub(crate) animate: MCByte,
}

impl Animation {
	const ID: u8 = 18;
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
		Self {
			eid: MCInt::read(stream),
			animate: MCByte::read(stream),
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
		MCUByte::write(stream, MCUByte { value: Self::ID });
		MCInt::write(stream, data.eid);
		MCByte::write(stream, data.animate);
	}
}

pub(crate) struct EntityAction {
	pub(crate) eid: MCInt,
	pub(crate) action: MCByte,
}

impl EntityAction {
	const ID: u8 = 19;
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
		Self {
			eid: MCInt::read(stream),
			action: MCByte::read(stream),
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
		MCUByte::write(stream, MCUByte { value: Self::ID });
		MCInt::write(stream, data.eid);
		MCByte::write(stream, data.action);
	}
}

pub(crate) struct NamedEntitySpawn {
	pub(crate) eid: MCInt,
	pub(crate) player_name: MCString16,
	pub(crate) x: MCInt,
	pub(crate) y: MCInt,
	pub(crate) z: MCInt,
	pub(crate) rotation: MCByte,
	pub(crate) pitch: MCByte,
	pub(crate) current_item: MCShort,
}

impl NamedEntitySpawn {
	const ID: u8 = 20;
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
		Self {
			eid: MCInt::read(stream),
			player_name: MCString16::read(stream),
			x: MCInt::read(stream),
			y: MCInt::read(stream),
			z: MCInt::read(stream),
			rotation: MCByte::read(stream),
			pitch: MCByte::read(stream),
			current_item: MCShort::read(stream),
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
		MCUByte::write(stream, MCUByte { value: Self::ID });
		MCInt::write(stream, data.eid);
		MCString16::write(stream, data.player_name);
		MCInt::write(stream, data.x);
		MCInt::write(stream, data.y);
		MCInt::write(stream, data.z);
		MCByte::write(stream, data.rotation);
		MCByte::write(stream, data.pitch);
		MCShort::write(stream, data.current_item);
	}
}

pub(crate) struct PickupSpawn {
	pub(crate) eid: MCInt,
	pub(crate) item: MCShort,
	pub(crate) count: MCByte,
	pub(crate) damage_or_data: MCShort,
	pub(crate) x: MCInt,
	pub(crate) y: MCInt,
	pub(crate) z: MCInt,
	pub(crate) rotation: MCByte,
	pub(crate) pitch: MCByte,
	pub(crate) roll: MCByte,
}

impl PickupSpawn {
	const ID: u8 = 21;
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
		Self {
			eid: MCInt::read(stream),
			item: MCShort::read(stream),
			count: MCByte::read(stream),
			damage_or_data: MCShort::read(stream),
			x: MCInt::read(stream),
			y: MCInt::read(stream),
			z: MCInt::read(stream),
			rotation: MCByte::read(stream),
			pitch: MCByte::read(stream),
			roll: MCByte::read(stream),
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
		MCUByte::write(stream, MCUByte { value: Self::ID });
		MCInt::write(stream, data.eid);
		MCShort::write(stream, data.item);
		MCByte::write(stream, data.count);
		MCShort::write(stream, data.damage_or_data);
		MCInt::write(stream, data.x);
		MCInt::write(stream, data.y);
		MCInt::write(stream, data.z);
		MCByte::write(stream, data.rotation);
		MCByte::write(stream, data.pitch);
		MCByte::write(stream, data.roll);
	}
}

pub(crate) struct CollectItem {
	pub(crate) collected_eid_1: MCInt,
	pub(crate) collected_eid_2: MCInt,
}

impl CollectItem {
	const ID: u8 = 22;
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
		Self {
			collected_eid_1: MCInt::read(stream),
			collected_eid_2: MCInt::read(stream),
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
		MCUByte::write(stream, MCUByte { value: Self::ID });
		MCInt::write(stream, data.collected_eid_1);
		MCInt::write(stream, data.collected_eid_2);
	}
}

pub(crate) struct AddObjectorVehicle {
	pub(crate) eid: MCInt,
	pub(crate) entity_type: MCByte,
	pub(crate) x: MCInt,
	pub(crate) y: MCInt,
	pub(crate) z: MCInt,
	pub(crate) unknown_flag: MCInt,
	pub(crate) unknown_1: MCShort,
	pub(crate) unknown_2: MCShort,
	pub(crate) unknown_3: MCShort,
}

impl AddObjectorVehicle {
	const ID: u8 = 23;
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
		Self {
			eid: MCInt::read(stream),
			entity_type: MCByte::read(stream),
			x: MCInt::read(stream),
			y: MCInt::read(stream),
			z: MCInt::read(stream),
			unknown_flag: MCInt::read(stream),
			unknown_1: MCShort::read(stream),
			unknown_2: MCShort::read(stream),
			unknown_3: MCShort::read(stream),
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
		MCUByte::write(stream, MCUByte { value: Self::ID });
		MCInt::write(stream, data.eid);
		MCByte::write(stream, data.entity_type);
		MCInt::write(stream, data.x);
		MCInt::write(stream, data.y);
		MCInt::write(stream, data.z);
		MCInt::write(stream, data.unknown_flag);
		MCShort::write(stream, data.unknown_1);
		MCShort::write(stream, data.unknown_2);
		MCShort::write(stream, data.unknown_3);
	}
}

pub(crate) struct MobSpawn {
	pub(crate) eid: MCInt,
	pub(crate) entity_type: MCByte,
	pub(crate) x: MCInt,
	pub(crate) y: MCInt,
	pub(crate) z: MCInt,
	pub(crate) yaw: MCByte,
	pub(crate) pitch: MCByte,
	pub(crate) data_stream: MCMetadata,
}

impl MobSpawn {
	const ID: u8 = 24;
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
		Self {
			eid: MCInt::read(stream),
			entity_type: MCByte::read(stream),
			x: MCInt::read(stream),
			y: MCInt::read(stream),
			z: MCInt::read(stream),
			yaw: MCByte::read(stream),
			pitch: MCByte::read(stream),
			data_stream: MCMetadata::read(stream),
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
		MCUByte::write(stream, MCUByte { value: Self::ID });
		MCInt::write(stream, data.eid);
		MCByte::write(stream, data.entity_type);
		MCInt::write(stream, data.x);
		MCInt::write(stream, data.y);
		MCInt::write(stream, data.z);
		MCByte::write(stream, data.yaw);
		MCByte::write(stream, data.pitch);
		MCMetadata::write(stream, data.data_stream);
	}
}

pub(crate) struct Painting {
	pub(crate) entity_id: MCInt,
	pub(crate) titile: MCString16,
	pub(crate) x: MCInt,
	pub(crate) y: MCInt,
	pub(crate) z: MCInt,
	pub(crate) direction: MCInt,
}

impl Painting {
	const ID: u8 = 25;
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
		Self {
			entity_id: MCInt::read(stream),
			titile: MCString16::read(stream),
			x: MCInt::read(stream),
			y: MCInt::read(stream),
			z: MCInt::read(stream),
			direction: MCInt::read(stream),
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
		MCUByte::write(stream, MCUByte { value: Self::ID });
		MCInt::write(stream, data.entity_id);
		MCString16::write(stream, data.titile);
		MCInt::write(stream, data.x);
		MCInt::write(stream, data.y);
		MCInt::write(stream, data.z);
		MCInt::write(stream, data.direction);
	}
}

pub(crate) struct StanceUpdate {
	pub(crate) unknown_1: MCFloat,
	pub(crate) unknown_2: MCFloat,
	pub(crate) unknown_3: MCFloat,
	pub(crate) unknown_4: MCFloat,
	pub(crate) unknown_5: MCBool,
	pub(crate) unknown_6: MCBool,
}

impl StanceUpdate {
	const ID: u8 = 27;
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
		Self {
			unknown_1: MCFloat::read(stream),
			unknown_2: MCFloat::read(stream),
			unknown_3: MCFloat::read(stream),
			unknown_4: MCFloat::read(stream),
			unknown_5: MCBool::read(stream),
			unknown_6: MCBool::read(stream),
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
		MCUByte::write(stream, MCUByte { value: Self::ID });
		MCFloat::write(stream, data.unknown_1);
		MCFloat::write(stream, data.unknown_2);
		MCFloat::write(stream, data.unknown_3);
		MCFloat::write(stream, data.unknown_4);
		MCBool::write(stream, data.unknown_5);
		MCBool::write(stream, data.unknown_6);
	}
}

pub(crate) struct EntityVelocity {
	pub(crate) entity_id: MCInt,
	pub(crate) velocity_x: MCShort,
	pub(crate) velocity_y: MCShort,
	pub(crate) velocity_z: MCShort,
}

impl EntityVelocity {
	const ID: u8 = 28;
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
		Self {
			entity_id: MCInt::read(stream),
			velocity_x: MCShort::read(stream),
			velocity_y: MCShort::read(stream),
			velocity_z: MCShort::read(stream),
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
		MCUByte::write(stream, MCUByte { value: Self::ID });
		MCInt::write(stream, data.entity_id);
		MCShort::write(stream, data.velocity_x);
		MCShort::write(stream, data.velocity_y);
		MCShort::write(stream, data.velocity_z);
	}
}

pub(crate) struct DestroyEntity {
	pub(crate) eid: MCInt,
}

impl DestroyEntity {
	const ID: u8 = 29;
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
		Self {
			eid: MCInt::read(stream),
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
		MCUByte::write(stream, MCUByte { value: Self::ID });
		MCInt::write(stream, data.eid);
	}
}

pub(crate) struct Entity {
	pub(crate) eid: MCInt,
}

impl Entity {
	const ID: u8 = 30;
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
		Self {
			eid: MCInt::read(stream),
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
		MCUByte::write(stream, MCUByte { value: Self::ID });
		MCInt::write(stream, data.eid);
	}
}

pub(crate) struct EntityRelativeMove {
	pub(crate) eid: MCInt,
	pub(crate) dx: MCByte,
	pub(crate) dy: MCByte,
	pub(crate) dz: MCByte,
}

impl EntityRelativeMove {
	const ID: u8 = 31;
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
		Self {
			eid: MCInt::read(stream),
			dx: MCByte::read(stream),
			dy: MCByte::read(stream),
			dz: MCByte::read(stream),
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
		MCUByte::write(stream, MCUByte { value: Self::ID });
		MCInt::write(stream, data.eid);
		MCByte::write(stream, data.dx);
		MCByte::write(stream, data.dy);
		MCByte::write(stream, data.dz);
	}
}

pub(crate) struct EntityLook {
	pub(crate) eid: MCInt,
	pub(crate) yaw: MCByte,
	pub(crate) pitch: MCByte,
}

impl EntityLook {
	const ID: u8 = 32;
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
		Self {
			eid: MCInt::read(stream),
			yaw: MCByte::read(stream),
			pitch: MCByte::read(stream),
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
		MCUByte::write(stream, MCUByte { value: Self::ID });
		MCInt::write(stream, data.eid);
		MCByte::write(stream, data.yaw);
		MCByte::write(stream, data.pitch);
	}
}

pub(crate) struct EntityLookandRelativeMove {
	pub(crate) eid: MCInt,
	pub(crate) dx: MCByte,
	pub(crate) dy: MCByte,
	pub(crate) dz: MCByte,
	pub(crate) yaw: MCByte,
	pub(crate) pitch: MCByte,
}

impl EntityLookandRelativeMove {
	const ID: u8 = 33;
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
		Self {
			eid: MCInt::read(stream),
			dx: MCByte::read(stream),
			dy: MCByte::read(stream),
			dz: MCByte::read(stream),
			yaw: MCByte::read(stream),
			pitch: MCByte::read(stream),
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
		MCUByte::write(stream, MCUByte { value: Self::ID });
		MCInt::write(stream, data.eid);
		MCByte::write(stream, data.dx);
		MCByte::write(stream, data.dy);
		MCByte::write(stream, data.dz);
		MCByte::write(stream, data.yaw);
		MCByte::write(stream, data.pitch);
	}
}

pub(crate) struct EntityTeleport {
	pub(crate) eid: MCInt,
	pub(crate) x: MCInt,
	pub(crate) y: MCInt,
	pub(crate) z: MCInt,
	pub(crate) yaw: MCByte,
	pub(crate) pitch: MCByte,
}

impl EntityTeleport {
	const ID: u8 = 34;
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
		Self {
			eid: MCInt::read(stream),
			x: MCInt::read(stream),
			y: MCInt::read(stream),
			z: MCInt::read(stream),
			yaw: MCByte::read(stream),
			pitch: MCByte::read(stream),
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
		MCUByte::write(stream, MCUByte { value: Self::ID });
		MCInt::write(stream, data.eid);
		MCInt::write(stream, data.x);
		MCInt::write(stream, data.y);
		MCInt::write(stream, data.z);
		MCByte::write(stream, data.yaw);
		MCByte::write(stream, data.pitch);
	}
}

pub(crate) struct EntityStatus {
	pub(crate) entity_id: MCInt,
	pub(crate) entity_status: MCByte,
}

impl EntityStatus {
	const ID: u8 = 38;
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
		Self {
			entity_id: MCInt::read(stream),
			entity_status: MCByte::read(stream),
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
		MCUByte::write(stream, MCUByte { value: Self::ID });
		MCInt::write(stream, data.entity_id);
		MCByte::write(stream, data.entity_status);
	}
}

pub(crate) struct AttachEntity {
	pub(crate) entity_id: MCInt,
	pub(crate) vehicle_id: MCInt,
}

impl AttachEntity {
	const ID: u8 = 39;
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
		Self {
			entity_id: MCInt::read(stream),
			vehicle_id: MCInt::read(stream),
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
		MCUByte::write(stream, MCUByte { value: Self::ID });
		MCInt::write(stream, data.entity_id);
		MCInt::write(stream, data.vehicle_id);
	}
}

pub(crate) struct Entitymetadata {
	pub(crate) entity_id: MCInt,
	pub(crate) entity_metadata: MCMetadata,
}

impl Entitymetadata {
	const ID: u8 = 40;
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
		Self {
			entity_id: MCInt::read(stream),
			entity_metadata: MCMetadata::read(stream),
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
		MCUByte::write(stream, MCUByte { value: Self::ID });
		MCInt::write(stream, data.entity_id);
		MCMetadata::write(stream, data.entity_metadata);
	}
}

pub(crate) struct PreChunk {
	pub(crate) x: MCInt,
	pub(crate) z: MCInt,
	pub(crate) mode: MCBool,
}

impl PreChunk {
	const ID: u8 = 50;
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
		Self {
			x: MCInt::read(stream),
			z: MCInt::read(stream),
			mode: MCBool::read(stream),
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
		MCUByte::write(stream, MCUByte { value: Self::ID });
		MCInt::write(stream, data.x);
		MCInt::write(stream, data.z);
		MCBool::write(stream, data.mode);
	}
}

pub(crate) struct MapChunk {
	pub(crate) x: MCInt,
	pub(crate) y: MCShort,
	pub(crate) z: MCInt,
	pub(crate) size_x: MCByte,
	pub(crate) size_y: MCByte,
	pub(crate) size_z: MCByte,
	pub(crate) map_chunk: MCMapChunk,
}

impl MapChunk {
	const ID: u8 = 51;
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
		Self {
			x: MCInt::read(stream),
			y: MCShort::read(stream),
			z: MCInt::read(stream),
			size_x: MCByte::read(stream),
			size_y: MCByte::read(stream),
			size_z: MCByte::read(stream),
			map_chunk: MCMapChunk::read(stream),
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
		MCUByte::write(stream, MCUByte { value: Self::ID });
		MCInt::write(stream, data.x);
		MCShort::write(stream, data.y);
		MCInt::write(stream, data.z);
		MCByte::write(stream, data.size_x);
		MCByte::write(stream, data.size_y);
		MCByte::write(stream, data.size_z);
		MCMapChunk::write(stream, data.map_chunk);
	}
}

pub(crate) struct MultiBlockChange {
	pub(crate) chunk_x: MCInt,
	pub(crate) chunk_z: MCInt,
	pub(crate) block_update_array: MCBlockUpdateArray,
}

impl MultiBlockChange {
	const ID: u8 = 52;
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
		Self {
			chunk_x: MCInt::read(stream),
			chunk_z: MCInt::read(stream),
			block_update_array: MCBlockUpdateArray::read(stream),
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
		MCUByte::write(stream, MCUByte { value: Self::ID });
		MCInt::write(stream, data.chunk_x);
		MCInt::write(stream, data.chunk_z);
		MCBlockUpdateArray::write(stream, data.block_update_array);
	}
}

pub(crate) struct BlockChange {
	pub(crate) x: MCInt,
	pub(crate) y: MCByte,
	pub(crate) z: MCInt,
	pub(crate) block_id: MCByte,
	pub(crate) block_metadata: MCByte,
}

impl BlockChange {
	const ID: u8 = 53;
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
		Self {
			x: MCInt::read(stream),
			y: MCByte::read(stream),
			z: MCInt::read(stream),
			block_id: MCByte::read(stream),
			block_metadata: MCByte::read(stream),
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
		MCUByte::write(stream, MCUByte { value: Self::ID });
		MCInt::write(stream, data.x);
		MCByte::write(stream, data.y);
		MCInt::write(stream, data.z);
		MCByte::write(stream, data.block_id);
		MCByte::write(stream, data.block_metadata);
	}
}

pub(crate) struct BlockAction {
	pub(crate) x: MCInt,
	pub(crate) y: MCShort,
	pub(crate) z: MCInt,
	pub(crate) instrument_type: MCByte,
	pub(crate) pitch: MCByte,
}

impl BlockAction {
	const ID: u8 = 54;
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
		Self {
			x: MCInt::read(stream),
			y: MCShort::read(stream),
			z: MCInt::read(stream),
			instrument_type: MCByte::read(stream),
			pitch: MCByte::read(stream),
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
		MCUByte::write(stream, MCUByte { value: Self::ID });
		MCInt::write(stream, data.x);
		MCShort::write(stream, data.y);
		MCInt::write(stream, data.z);
		MCByte::write(stream, data.instrument_type);
		MCByte::write(stream, data.pitch);
	}
}

pub(crate) struct Explosion {
	pub(crate) x: MCDouble,
	pub(crate) y: MCDouble,
	pub(crate) z: MCDouble,
	pub(crate) unknown: MCFloat,
	pub(crate) explosion_update_array: MCExplosionUpdate,
}

impl Explosion {
	const ID: u8 = 60;
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
		Self {
			x: MCDouble::read(stream),
			y: MCDouble::read(stream),
			z: MCDouble::read(stream),
			unknown: MCFloat::read(stream),
			explosion_update_array: MCExplosionUpdate::read(stream),
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
		MCUByte::write(stream, MCUByte { value: Self::ID });
		MCDouble::write(stream, data.x);
		MCDouble::write(stream, data.y);
		MCDouble::write(stream, data.z);
		MCFloat::write(stream, data.unknown);
		MCExplosionUpdate::write(stream, data.explosion_update_array);
	}
}

pub(crate) struct SoundEffect {
	pub(crate) effect_id: MCInt,
	pub(crate) x: MCInt,
	pub(crate) y: MCByte,
	pub(crate) z: MCInt,
	pub(crate) sound_data: MCInt,
}

impl SoundEffect {
	const ID: u8 = 61;
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
		Self {
			effect_id: MCInt::read(stream),
			x: MCInt::read(stream),
			y: MCByte::read(stream),
			z: MCInt::read(stream),
			sound_data: MCInt::read(stream),
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
		MCUByte::write(stream, MCUByte { value: Self::ID });
		MCInt::write(stream, data.effect_id);
		MCInt::write(stream, data.x);
		MCByte::write(stream, data.y);
		MCInt::write(stream, data.z);
		MCInt::write(stream, data.sound_data);
	}
}

pub(crate) struct NeworInvalidState {
	pub(crate) reason: MCByte,
}

impl NeworInvalidState {
	const ID: u8 = 70;
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
		Self {
			reason: MCByte::read(stream),
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
		MCUByte::write(stream, MCUByte { value: Self::ID });
		MCByte::write(stream, data.reason);
	}
}

pub(crate) struct Thunderbolt {
	pub(crate) entity_id: MCInt,
	pub(crate) unknown: MCBool,
	pub(crate) x: MCInt,
	pub(crate) y: MCInt,
	pub(crate) z: MCInt,
}

impl Thunderbolt {
	const ID: u8 = 71;
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
		Self {
			entity_id: MCInt::read(stream),
			unknown: MCBool::read(stream),
			x: MCInt::read(stream),
			y: MCInt::read(stream),
			z: MCInt::read(stream),
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
		MCUByte::write(stream, MCUByte { value: Self::ID });
		MCInt::write(stream, data.entity_id);
		MCBool::write(stream, data.unknown);
		MCInt::write(stream, data.x);
		MCInt::write(stream, data.y);
		MCInt::write(stream, data.z);
	}
}

pub(crate) struct OpenWindow {
	pub(crate) window_id: MCByte,
	pub(crate) inventory_type: MCByte,
	pub(crate) window_title: MCString8,
	pub(crate) number_of_slots: MCByte,
}

impl OpenWindow {
	const ID: u8 = 100;
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
		Self {
			window_id: MCByte::read(stream),
			inventory_type: MCByte::read(stream),
			window_title: MCString8::read(stream),
			number_of_slots: MCByte::read(stream),
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
		MCUByte::write(stream, MCUByte { value: Self::ID });
		MCByte::write(stream, data.window_id);
		MCByte::write(stream, data.inventory_type);
		MCString8::write(stream, data.window_title);
		MCByte::write(stream, data.number_of_slots);
	}
}

pub(crate) struct CloseWindow {}

impl CloseWindow {
	const ID: u8 = 101;
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
		Self {}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
		MCUByte::write(stream, MCUByte { value: Self::ID });
	}
}

pub(crate) struct WindowClick {
	pub(crate) window_id: MCByte,
	pub(crate) slot: MCShort,
	pub(crate) right_click: MCBool,
	pub(crate) action_number: MCShort,
	pub(crate) shift: MCBool,
	pub(crate) item: MCItem,
}

impl WindowClick {
	const ID: u8 = 102;
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
		Self {
			window_id: MCByte::read(stream),
			slot: MCShort::read(stream),
			right_click: MCBool::read(stream),
			action_number: MCShort::read(stream),
			shift: MCBool::read(stream),
			item: MCItem::read(stream),
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
		MCUByte::write(stream, MCUByte { value: Self::ID });
		MCByte::write(stream, data.window_id);
		MCShort::write(stream, data.slot);
		MCBool::write(stream, data.right_click);
		MCShort::write(stream, data.action_number);
		MCBool::write(stream, data.shift);
		MCItem::write(stream, data.item);
	}
}

pub(crate) struct SetSlot {
	pub(crate) window_id: MCByte,
	pub(crate) slot: MCShort,
	pub(crate) item: MCItem,
}

impl SetSlot {
	const ID: u8 = 103;
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
		Self {
			window_id: MCByte::read(stream),
			slot: MCShort::read(stream),
			item: MCItem::read(stream),
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
		MCUByte::write(stream, MCUByte { value: Self::ID });
		MCByte::write(stream, data.window_id);
		MCShort::write(stream, data.slot);
		MCItem::write(stream, data.item);
	}
}

pub(crate) struct WindowItems {
	pub(crate) window_id: MCByte,
	pub(crate) payload: MCInventoryPayload,
}

impl WindowItems {
	const ID: u8 = 104;
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
		Self {
			window_id: MCByte::read(stream),
			payload: MCInventoryPayload::read(stream),
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
		MCUByte::write(stream, MCUByte { value: Self::ID });
		MCByte::write(stream, data.window_id);
		MCInventoryPayload::write(stream, data.payload);
	}
}

pub(crate) struct UpdateProgressBar {
	pub(crate) window_id: MCByte,
	pub(crate) progress_bar: MCShort,
	pub(crate) value: MCShort,
}

impl UpdateProgressBar {
	const ID: u8 = 105;
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
		Self {
			window_id: MCByte::read(stream),
			progress_bar: MCShort::read(stream),
			value: MCShort::read(stream),
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
		MCUByte::write(stream, MCUByte { value: Self::ID });
		MCByte::write(stream, data.window_id);
		MCShort::write(stream, data.progress_bar);
		MCShort::write(stream, data.value);
	}
}

pub(crate) struct Transaction {
	pub(crate) window_id: MCByte,
	pub(crate) action_number: MCShort,
	pub(crate) accepted: MCBool,
}

impl Transaction {
	const ID: u8 = 106;
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
		Self {
			window_id: MCByte::read(stream),
			action_number: MCShort::read(stream),
			accepted: MCBool::read(stream),
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
		MCUByte::write(stream, MCUByte { value: Self::ID });
		MCByte::write(stream, data.window_id);
		MCShort::write(stream, data.action_number);
		MCBool::write(stream, data.accepted);
	}
}

pub(crate) struct UpdateSign {
	pub(crate) x: MCInt,
	pub(crate) y: MCShort,
	pub(crate) z: MCInt,
	pub(crate) text1: MCString16,
	pub(crate) text2: MCString16,
	pub(crate) text3: MCString16,
	pub(crate) text4: MCString16,
}

impl UpdateSign {
	const ID: u8 = 130;
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
		Self {
			x: MCInt::read(stream),
			y: MCShort::read(stream),
			z: MCInt::read(stream),
			text1: MCString16::read(stream),
			text2: MCString16::read(stream),
			text3: MCString16::read(stream),
			text4: MCString16::read(stream),
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
		MCUByte::write(stream, MCUByte { value: Self::ID });
		MCInt::write(stream, data.x);
		MCShort::write(stream, data.y);
		MCInt::write(stream, data.z);
		MCString16::write(stream, data.text1);
		MCString16::write(stream, data.text2);
		MCString16::write(stream, data.text3);
		MCString16::write(stream, data.text4);
	}
}

pub(crate) struct ItemData {
	pub(crate) item_type: MCShort,
	pub(crate) item_id: MCShort,
	pub(crate) text_length: MCByte,
	pub(crate) text: MCString8,
}

impl ItemData {
	const ID: u8 = 131;
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
		Self {
			item_type: MCShort::read(stream),
			item_id: MCShort::read(stream),
			text_length: MCByte::read(stream),
			text: MCString8::read(stream),
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
		MCUByte::write(stream, MCUByte { value: Self::ID });
		MCShort::write(stream, data.item_type);
		MCShort::write(stream, data.item_id);
		MCByte::write(stream, data.text_length);
		MCString8::write(stream, data.text);
	}
}

pub(crate) struct IncrementStatistic {
	pub(crate) statistic_id: MCInt,
	pub(crate) amount: MCByte,
}

impl IncrementStatistic {
	const ID: u8 = 200;
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
		Self {
			statistic_id: MCInt::read(stream),
			amount: MCByte::read(stream),
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
		MCUByte::write(stream, MCUByte { value: Self::ID });
		MCInt::write(stream, data.statistic_id);
		MCByte::write(stream, data.amount);
	}
}

pub(crate) struct DisconnectorKick {
	pub(crate) reason: MCString16,
}

impl DisconnectorKick {
	const ID: u8 = 255;
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
		Self {
			reason: MCString16::read(stream),
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
		MCUByte::write(stream, MCUByte { value: Self::ID });
		MCString16::write(stream, data.reason);
	}
}
