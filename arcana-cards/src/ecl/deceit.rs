//! Deceit — `{4}{U/B}{U/B}` 5/5 Elemental Incarnation.
//! "When this creature enters, if {U}{U} was spent to cast it, return
//!  up to one other target nonland permanent to its owner's hand."
//! "When this creature enters, if {B}{B} was spent to cast it, target
//!  opponent reveals their hand. You choose a nonland card from it.
//!  That player discards that card."
//! "Evoke {U/B}{U/B}"
//!
//! Wired: both ETB effects (a bounce of a nonland permanent; a targeted
//! opponent discard). The "if {U}{U}/{B}{B} was spent" intervening-if is
//! GAP'd — there is no condition predicate for mana spent to cast, so
//! the triggers fire unconditionally. Evoke is not an available keyword
//! (GAP'd).

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Deceit");
    let elemental = reg.interner_mut().intern("Elemental");
    let incarnation = reg.interner_mut().intern("Incarnation");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(incarnation);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U/B}{U/B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    // GAP: keyword — Evoke {U/B}{U/B} (not an available KeywordAbility).
    reg.register(
        CardDefinition::new(name, chars)
            // GAP: intervening-if "if {U}{U} was spent to cast it" — no
            // mana-spent condition predicate; the trigger fires always.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: bounce_nonland_permanent,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent().without_types(TypeLine::LAND.into()),
                    ),
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
            })
            // GAP: intervening-if "if {B}{B} was spent to cast it" — no
            // mana-spent condition predicate; the trigger fires always.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: opponent_discards_chosen,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_player()],
            }),
    )
}

fn bounce_nonland_permanent(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::ReturnToHand { target: *id }]
}

fn opponent_discards_chosen(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Player(p) = target else { return Vec::new(); };
    // You choose which (nonland) card the targeted opponent discards.
    // The nonland restriction isn't expressible on Discard (no filter
    // field) — minor fidelity gap.
    vec![Effect::Discard {
        player: *p,
        count: 1,
        choice: DiscardChoice::OpponentChooses,
    }]
}
