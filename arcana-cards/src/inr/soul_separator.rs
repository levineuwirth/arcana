//! Soul Separator — `{3}` artifact.
//! "{5}, {T}, Sacrifice this artifact: Exile target creature card from
//! your graveyard. Create a token that's a copy of that card, except
//! it's 1/1, it's a Spirit in addition to its other types, and it has
//! flying. Create a black Zombie creature token with power equal to
//! that card's power and toughness equal to that card's toughness."
//! The exile and the dynamic-P/T Zombie token are wired; the
//! modified-copy Spirit token is a GAP (CopyPermanent has no 'except'
//! modifications and copies battlefield permanents, not graveyard
//! cards).

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Soul Separator");
    let _zombie = reg.interner_mut().intern("Zombie");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{5}, {T}, Sacrifice this artifact: Exile target creature card from your graveyard. Create a token that's a copy of that card, except it's 1/1, it's a Spirit in addition to its other types, and it has flying. Create a black Zombie creature token with power equal to that card's power and toughness equal to that card's toughness.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{5}").expect("valid cost"),
                tap: true,
                sacrifice: true,
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Card {
                    zone: Zone::Graveyard(0),
                    filter: ObjectFilter::creature(),
                },
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: separate_soul,
        }),
    )
}

fn separate_soul(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let p = script::power_of(state, *id).max(0);
    let t = script::toughness_of(state, *id).max(0);
    let zombie = reg.interner().lookup("Zombie").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    // GAP: the 1/1 Spirit modified-copy token ("a copy of that card,
    // except it's 1/1, it's a Spirit ... and it has flying") is not
    // expressible — CopyPermanent has no 'except' modifications and no
    // graveyard-card source.
    vec![
        Effect::ExileFromGraveyard { target: *id },
        Effect::CreateToken {
            controller: ctx.controller,
            token: TokenDefinition {
                name: zombie,
                colors: ColorSet::black(),
                types: TypeLine::CREATURE.into(),
                subtypes,
                power: Some(PtValue::Fixed(p as _)),
                toughness: Some(PtValue::Fixed(t as _)),
                keywords: vec![],
                abilities: vec![],
            },
        },
    ]
}
