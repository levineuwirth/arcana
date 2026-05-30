//! Ludevic's Test Subject // Ludevic's Abomination
//!
//! Front: {1}{U} Creature — Lizard Egg 0/3, Defender.
//! Activated: {1}{U}: Put a hatchling counter on this creature. Then if there are
//! five or more hatchling counters on it, remove all of them and transform it.
//! Back: Ludevic's Abomination — Creature — Lizard Horror, Trample (no P/T on the
//! oracle back face printed; treated as */*, we use 13/13 as the canonical oracle stat).
//!
//! GAP: "hatchling counter" has no dedicated CounterKind variant; using Named("hatchling").
//! GAP: The "if 5+ hatchling counters, remove all then transform" conditional gate
//! is not expressible — we emit AddCounters + Transform unconditionally on every
//! activation (missing the count-check and the remove-all step).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ludevic's Test Subject");

    let lizard_sub = reg.interner_mut().intern("Lizard");
    let egg_sub = reg.interner_mut().intern("Egg");
    let mut front_subtypes = SubtypeSet::default();
    front_subtypes.0.insert(lizard_sub);
    front_subtypes.0.insert(egg_sub);

    // Pre-intern the Named counter kind so it can be referenced at activation time
    let _hatchling_name = reg.interner_mut().intern("hatchling");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes: front_subtypes,
        keywords: vec![KeywordAbility::Defender],
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Ludevic's Abomination");
    let lizard_back = reg.interner_mut().intern("Lizard");
    let horror_sub = reg.interner_mut().intern("Horror");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(lizard_back);
    back_subtypes.0.insert(horror_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            keywords: vec![KeywordAbility::Trample],
            power: Some(PtValue::Fixed(13)),
            toughness: Some(PtValue::Fixed(13)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // {1}{U}: Add a hatchling counter; then (GAP) transform.
            // face_gate: Some(0) — only active on front face.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{U}: Put a hatchling counter on this creature. Then if there are five or more hatchling counters on it, remove all of them and transform it.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{U}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(0),
                effect: hatchling_activate,
            }),
    )
}

/// {1}{U}: Add a hatchling counter, then transform.
/// GAP: Missing the "if 5 or more" count check and the "remove all" step.
fn hatchling_activate(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let hatchling = reg
        .interner()
        .lookup("hatchling")
        .expect("hatchling interned during register");
    vec![
        Effect::AddCounters {
            target: ctx.source,
            kind: CounterKind::Named(hatchling),
            count: 1,
        },
        // GAP: should only transform when hatchling counter count reaches 5,
        // and should remove all hatchling counters first.
        Effect::Transform { target: ctx.source },
    ]
}
