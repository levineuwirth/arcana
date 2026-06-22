//! Surge Engine — `{2}` 3/2 Artifact Creature — Construct with
//! Defender.
//!
//! Defender.
//! {U}: This creature loses defender and gains "This creature can't be
//!   blocked."
//! {2}{U}: This creature becomes blue and has base power and toughness
//!   5/4. Activate only if this creature doesn't have defender.
//! {4}{U}{U}: Draw three cards. Activate only if this creature is blue
//!   and only once.
//!
//! All three activated abilities are wired. Per-ability GAPs:
//!   * #1: "loses defender" has no single-keyword-removal primitive in
//!     the demonstrated surface; the "can't be blocked" grant IS wired.
//!   * #2: the "only if it doesn't have defender" activation gate has
//!     no expressible predicate; the become-blue + base-5/4 IS wired
//!     (permanently, matching the no-stated-duration text).
//!   * #3: the "only if blue" gate isn't expressible; "only once" is
//!     approximated by `once_per_turn`; the draw IS wired.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Surge Engine");
    let construct = reg.interner_mut().intern("Construct");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(construct);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![arcana_core::effects::KeywordAbility::Defender],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                // GAP: "loses defender" — no single-keyword removal.
                text: "{U}: This creature loses defender and gains \
                       \"This creature can't be blocked.\""
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{U}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: gain_unblockable,
            })
            .with_activated_ability(ActivatedAbilityDef {
                // GAP: "Activate only if this creature doesn't have
                // defender." — gate not expressible.
                text: "{2}{U}: This creature becomes blue and has base \
                       power and toughness 5/4."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{U}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: become_blue_5_4,
            })
            .with_activated_ability(ActivatedAbilityDef {
                // GAP: "Activate only if this creature is blue." — gate
                // not expressible. "Only once" approximated by
                // once_per_turn.
                text: "{4}{U}{U}: Draw three cards.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{4}{U}{U}").expect("valid cost"),
                    once_per_turn: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: draw_three,
            }),
    )
}

fn gain_unblockable(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::CantBeBlocked {
        target: ctx.source,
        duration: Duration::WhileSourceOnBattlefield,
    }]
}

fn become_blue_5_4(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::SetColor {
            target: ctx.source,
            colors: ColorSet::blue(),
            duration: Duration::Permanent,
        },
        Effect::SetBasePT {
            target: ctx.source,
            power: 5,
            toughness: 4,
            duration: Duration::Permanent,
        },
    ]
}

fn draw_three(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards {
        player: ctx.controller,
        count: 3,
    }]
}
