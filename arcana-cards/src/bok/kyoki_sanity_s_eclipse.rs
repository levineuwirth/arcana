//! Kyoki, Sanity's Eclipse — `{4}{B}{B}` 6/4 legendary black Demon Spirit.
//! "Whenever you cast a Spirit or Arcane spell, target opponent exiles a card
//! from their hand."

use arcana_core::effects::{Effect, DiscardChoice};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount,
    TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kyoki, Sanity's Eclipse");
    let demon = reg.interner_mut().intern("Demon");
    let spirit = reg.interner_mut().intern("Spirit");
    let arcane = reg.interner_mut().intern("Arcane");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon);
    subtypes.0.insert(spirit);

    // Pre-intern Arcane so the SpellCast filter can match it via subtype
    let _arcane_interned = arcane;

    let spirit_or_arcane_filter = ObjectFilter {
        subtypes_any: Some(vec![spirit, arcane]),
        ..ObjectFilter::default()
    };

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(spirit_or_arcane_filter),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: opponent_exiles_from_hand,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Player,
                    count: TargetCount::Exactly(1),
                    controller: Some(ControllerConstraint::Opponent),
                }],
            }),
    )
}

fn opponent_exiles_from_hand(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Player(p) = target else { return Vec::new(); };
    // GAP: "exile a card from their hand" — the engine has Discard but not
    // ExileFromHand. Using Discard(OpponentChooses) as best available
    // approximation; the exile-from-hand variant is not in the effect catalog.
    vec![Effect::Discard {
        player: *p,
        count: 1,
        choice: DiscardChoice::ControllerChooses,
    }]
}
