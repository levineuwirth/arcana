//! A-Llanowar Loamspeaker — `{1}{G}` 1/3 Elf Druid.
//! {T}: Add one mana of any color. (Modeled as five mana abilities, one
//! per WUBRG color; the player picks the color by choosing which ability
//! to activate — the shared {T} cost taps the source so only one fires.)
//! {T}: Target land you control becomes a 4/4 Elemental creature with haste
//! until end of turn. It's still a land. Sorcery-speed.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("A-Llanowar Loamspeaker");
    let elf = reg.interner_mut().intern("Elf");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(druid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // "{T}: Add one mana of any color" — five mana abilities, one per color.
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(mana_ability("{T}: Add {W}.", add_white_mana))
            .with_activated_ability(mana_ability("{T}: Add {U}.", add_blue_mana))
            .with_activated_ability(mana_ability("{T}: Add {B}.", add_black_mana))
            .with_activated_ability(mana_ability("{T}: Add {R}.", add_red_mana))
            .with_activated_ability(mana_ability("{T}: Add {G}.", add_green_mana))
            .with_activated_ability(ActivatedAbilityDef {
            text: "{T}: Target land you control becomes a 4/4 Elemental creature with haste until end of turn. It's still a land. Activate only as a sorcery.".into(),
            cost: ActivationCost {
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::permanent()
                        .with_types(TypeLine::LAND.into())
                        .controlled_by(ControllerConstraint::You),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: animate_land,
        }),
    )
}

fn animate_land(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    let id = *id;
    // Becomes a 4/4 creature (additive type — still a land) with haste EOT.
    // GAP: the "Elemental" creature subtype is not added (no add-subtype
    // Effect in the catalog); the P/T, creature type, and haste are applied.
    vec![
        Effect::AddType {
            target: id,
            types: TypeLine::CREATURE.into(),
            duration: Duration::EndOfTurn,
        },
        Effect::SetBasePT {
            target: id,
            power: 4,
            toughness: 4,
            duration: Duration::EndOfTurn,
        },
        Effect::GrantKeyword {
            target: id,
            keyword: KeywordAbility::Haste,
            duration: Duration::EndOfTurn,
        },
    ]
}

/// `{T}: Add one mana of any color` — one tap-only mana ability per color.
fn mana_ability(
    text: &str,
    effect: fn(&GameState, &ActivationContext, &CardRegistry) -> Vec<Effect>,
) -> ActivatedAbilityDef {
    ActivatedAbilityDef {
        text: text.into(),
        cost: ActivationCost {
            tap: true,
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

fn add_one(ctx: &ActivationContext, color: ManaColor) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(color, ctx.source)],
    }]
}

fn add_white_mana(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    add_one(ctx, ManaColor::White)
}
fn add_blue_mana(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    add_one(ctx, ManaColor::Blue)
}
fn add_black_mana(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    add_one(ctx, ManaColor::Black)
}
fn add_red_mana(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    add_one(ctx, ManaColor::Red)
}
fn add_green_mana(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    add_one(ctx, ManaColor::Green)
}
