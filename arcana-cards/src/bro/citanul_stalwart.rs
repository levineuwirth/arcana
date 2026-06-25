//! Citanul Stalwart — `{G}` 1/1 green Elf Druid Soldier.
//! "{T}, Tap an untapped artifact or creature you control: Add one mana
//! of any color."
//! "Add one mana of any color" is modeled as five mana abilities, one per WUBRG
//! color; the shared {T} + tap-other cost means only one fires (command_tower
//! idiom). The "Tap an untapped artifact or creature you control" cost is
//! expressed via tap_other with an artifact-or-creature filter.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Citanul Stalwart");
    let elf = reg.interner_mut().intern("Elf");
    let druid = reg.interner_mut().intern("Druid");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(druid);
    subtypes.0.insert(soldier);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(mana_ability("{T}, Tap an untapped artifact or creature you control: Add {W}.", add_white))
            .with_activated_ability(mana_ability("{T}, Tap an untapped artifact or creature you control: Add {U}.", add_blue))
            .with_activated_ability(mana_ability("{T}, Tap an untapped artifact or creature you control: Add {B}.", add_black))
            .with_activated_ability(mana_ability("{T}, Tap an untapped artifact or creature you control: Add {R}.", add_red))
            .with_activated_ability(mana_ability("{T}, Tap an untapped artifact or creature you control: Add {G}.", add_green)),
    )
}

/// "Tap an untapped artifact or creature you control" — an artifact-or-creature
/// permanent you control as a tap-other cost (in addition to {T} on the source).
fn art_or_creature() -> ObjectFilter {
    ObjectFilter::permanent()
        .with_types_any(arcana_core::types::TypeLine(
            TypeLine::ARTIFACT | TypeLine::CREATURE,
        ))
        .controlled_by(ControllerConstraint::You)
        .untapped_only()
}

fn mana_ability(
    text: &str,
    effect: fn(&GameState, &ActivationContext, &CardRegistry) -> Vec<Effect>,
) -> ActivatedAbilityDef {
    ActivatedAbilityDef {
        text: text.into(),
        cost: ActivationCost {
            tap: true,
            tap_other: Some(art_or_creature()),
            tap_other_count: 1,
            ..ActivationCost::default()
        },
        target_requirements: Vec::new(),
        is_mana_ability: true,
        is_loyalty_ability: false,
        activation_zone: ActivationZone::Battlefield,
        is_instant_speed: false,
        face_gate: None,
        effect,
    }
}

fn add_white(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::White, ctx.source)],
    }]
}

fn add_blue(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Blue, ctx.source)],
    }]
}

fn add_black(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Black, ctx.source)],
    }]
}

fn add_red(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Red, ctx.source)],
    }]
}

fn add_green(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Green, ctx.source)],
    }]
}
