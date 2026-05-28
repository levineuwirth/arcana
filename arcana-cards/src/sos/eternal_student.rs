//! Eternal Student — `{3}{B}` 4/2 Creature — Zombie Warlock.
//! `{1}{B}, Exile this card from your graveyard: Create two 1/1 white and black Inkling creature tokens with flying.`

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Eternal Student");
    let zombie = reg.interner_mut().intern("Zombie");
    let warlock = reg.interner_mut().intern("Warlock");
    let _inkling = reg.interner_mut().intern("Inkling");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(warlock);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{B}, Exile this card from your graveyard: Create two 1/1 white and black Inkling creature tokens with flying.".into(),
                cost: ActivationCost { mana_cost: ManaCost::parse("{1}{B}").unwrap(), exile_self: true, ..ActivationCost::default() },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: create_inklings,
            }),
    )
}

fn create_inklings(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let inkling = reg.interner().lookup("Inkling").expect("Inkling interned");
    let mut ts = SubtypeSet::default();
    ts.0.insert(inkling);
    let make_token = || {
        let mut ts2 = SubtypeSet::default();
        ts2.0.insert(inkling);
        Effect::CreateToken {
            controller: ctx.controller,
            token: TokenDefinition {
                name: inkling,
                colors: ColorSet::white() | ColorSet::black(),
                types: TypeLine::CREATURE.into(),
                subtypes: ts2,
                power: Some(PtValue::Fixed(1)),
                toughness: Some(PtValue::Fixed(1)),
                keywords: vec![KeywordAbility::Flying],
                abilities: vec![],
            },
        }
    };
    vec![make_token(), make_token()]
}
