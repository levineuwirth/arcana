//! Kaervek, the Punisher — `{1}{B}{B}` 3/3 Legendary Human Warlock.
//! "Whenever you commit a crime, exile up to one target black card from your
//! graveyard and copy it. You may cast the copy. If you do, you lose 2 life."
//! GAP: No "crime" trigger condition in the engine (targeting opponents /
//! their permanents / cards in their graveyards). Using SpellCast with
//! Opponent filter as the closest approximation (partial — misses activated
//! ability targeting, etc.). Also GAP: "copy it, may cast the copy" is not
//! expressible; ExileFromGraveyard is used as a placeholder.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kaervek, the Punisher");
    let human = reg.interner_mut().intern("Human");
    let warlock = reg.interner_mut().intern("Warlock");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warlock);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — "whenever you commit a crime" has no engine variant.
                // Closest: whenever an opponent casts a spell (very partial approximation).
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: crime_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::new().with_colors(ColorSet::black()),
                    },
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
            }),
    )
}

fn crime_trigger(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    // GAP: "copy it, you may cast the copy" — no copy-from-graveyard-and-cast Effect variant.
    // Exile the targeted card as a best-effort partial.
    vec![
        Effect::ExileFromGraveyard { target: *id },
        Effect::LoseLife { player: trig.controller, amount: 2 },
    ]
}
