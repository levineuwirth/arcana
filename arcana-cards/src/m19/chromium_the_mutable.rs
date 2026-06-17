//! Chromium, the Mutable — `{4}{W}{U}{B}` 7/7 Legendary Elder Dragon.
//! Flash; "This spell can't be countered."; Flying; `Discard a card: until end of
//! turn, Chromium becomes a Human with base P/T 1/1, loses all abilities, and gains
//! hexproof. It can't be blocked this turn.`
//!
//! Flash and Flying are base keywords. "Can't be countered" is a static with no
//! Effect/keyword (GAP). The activated ability discards a card (discard_other) and
//! applies SetBasePT 1/1, LoseAllAbilities, grants Hexproof, and CantBeBlocked, all
//! until end of turn; "becomes a Human" (subtype change) is GAP'd (AddType handles
//! card types, not creature subtypes).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chromium, the Mutable");
    let elder = reg.interner_mut().intern("Elder");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elder);
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}{U}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(7)),
        keywords: vec![KeywordAbility::Flash, KeywordAbility::Flying],
        ..Default::default()
    };
    // GAP: "This spell can't be countered." — static, no Effect/keyword form.

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "Discard a card: Until end of turn, Chromium becomes a Human with base power and toughness 1/1, loses all abilities, and gains hexproof. It can't be blocked this turn."
                    .into(),
                cost: ActivationCost {
                    discard_other: Some(ObjectFilter::default()),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: become_human,
            }),
    )
}

fn become_human(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "becomes a Human" subtype change is not expressible (AddType is for
    // card types, not creature subtypes).
    vec![
        Effect::SetBasePT {
            target: ctx.source,
            power: 1,
            toughness: 1,
            duration: Duration::EndOfTurn,
        },
        Effect::LoseAllAbilities {
            target: ctx.source,
            duration: Duration::EndOfTurn,
        },
        Effect::GrantKeyword {
            target: ctx.source,
            keyword: KeywordAbility::Hexproof,
            duration: Duration::EndOfTurn,
        },
        Effect::CantBeBlocked {
            target: ctx.source,
            duration: Duration::EndOfTurn,
        },
    ]
}
