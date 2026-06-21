//! Tatsunari, Toad Rider — `{2}{B}` 3/3 legendary Human Ninja.
//!
//! Rules text:
//! * "Whenever you cast an enchantment spell, if you don't control a
//!   creature named Keimi, create Keimi, a legendary 3/3 black and
//!   green Frog creature token with 'Whenever you cast an enchantment
//!   spell, each opponent loses 1 life and you gain 1 life.'" —
//!   expressible: an enchantment-cast trigger, an intervening-if on not
//!   controlling Keimi, and a token carrying its own enchantment-cast
//!   triggered ability.
//! * "{1}{G/U}: Tatsunari and target Frog you control can't be blocked
//!   this turn except by creatures with flying or reach." — GAP'd: no
//!   "can't be blocked except by <quality>" conditional-evasion
//!   primitive (the plain CantBeBlocked would be strictly wrong).

use arcana_core::conditions;
use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PlayerId, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tatsunari, Toad Rider");
    let human = reg.interner_mut().intern("Human");
    let ninja = reg.interner_mut().intern("Ninja");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(ninja);
    // Pre-intern Keimi's name/subtype so the resolver can rebuild them.
    let _keimi = reg.interner_mut().intern("Keimi");
    let _frog = reg.interner_mut().intern("Frog");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: activated "{1}{G/U}: Tatsunari and target Frog you control
    // can't be blocked this turn except by creatures with flying or
    // reach." — no conditional-evasion ("except by flying or reach")
    // primitive.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(ObjectFilter::new().with_types(TypeLine::ENCHANTMENT.into())),
                    caster: ControllerConstraint::You,
                },
                intervening_if: Some(if_no_keimi),
                effect: make_keimi,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// "if you don't control a creature named Keimi".
fn if_no_keimi(s: &GameState, _src: ObjectId, you: PlayerId, reg: &CardRegistry) -> bool {
    let keimi = reg.interner().lookup("Keimi");
    let filter = ObjectFilter {
        name: keimi,
        ..ObjectFilter::creature().controlled_by(ControllerConstraint::You)
    };
    !conditions::you_control_a(s, you, &filter)
}

fn make_keimi(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let keimi_name = reg.interner().lookup("Keimi").unwrap_or_default();
    let mut keimi_subtypes = SubtypeSet::default();
    if let Some(f) = reg.interner().lookup("Frog") {
        keimi_subtypes.0.insert(f);
    }
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: keimi_name,
            colors: ColorSet::black() | ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes: keimi_subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(3)),
            keywords: vec![],
            abilities: vec![TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(
                        ObjectFilter::new().with_types(TypeLine::ENCHANTMENT.into()),
                    ),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: keimi_drain,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }],
        },
    }]
}

/// Keimi's ability: "each opponent loses 1 life and you gain 1 life."
fn keimi_drain(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects: Vec<Effect> = script::opponents(state, trig.controller)
        .into_iter()
        .map(|p| Effect::LoseLife { player: p, amount: 1 })
        .collect();
    effects.push(Effect::GainLife {
        player: trig.controller,
        amount: 1,
    });
    effects
}
