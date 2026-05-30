//! Thing in the Ice // Awoken Horror — `{1}{U}` Horror 0/4 with Defender.
//! Enters with four ice counters on it. Whenever you cast an instant or
//! sorcery spell, remove an ice counter from this creature. Then if it has
//! no ice counters on it, transform it.
//! Back face (Awoken Horror) 7/8: when it transforms, return all non-Horror
//! creatures to their owners' hands.
//!
//! GAP: "Then if it has no ice counters on it, transform" is a conditional
//!      on counter-count that cannot be expressed; counter removal fires but
//!      the conditional transform is omitted.
//! GAP: back-face-only triggered ability (bounce all non-Horror creatures
//!      on transform) not auto-installed; not modeled here.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, EntersWithSpec};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Thing in the Ice");
    let horror_sub = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(horror_sub);
    // Pre-intern "ice" for CounterKind::Named usage at resolve time.
    let ice_id = reg.interner_mut().intern("ice");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        keywords: vec![KeywordAbility::Defender],
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Awoken Horror");
    let kraken_sub = reg.interner_mut().intern("Kraken");
    let back_horror_sub = reg.interner_mut().intern("Horror");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(kraken_sub);
    back_subtypes.0.insert(back_horror_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(7)),
            toughness: Some(PtValue::Fixed(8)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::Named(ice_id),
                count: 4,
            })
            // Whenever you cast an instant or sorcery spell, remove an ice counter.
            // GAP: "Then if it has no ice counters on it, transform" omitted —
            //      counter-count conditional not expressible in TriggerCondition.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(
                        ObjectFilter::new().with_types_any(
                            TypeLine(TypeLine::INSTANT | TypeLine::SORCERY),
                        ),
                    ),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: on_instant_or_sorcery,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // GAP: back-face-only triggered ability not modeled
            // (When transforms into Awoken Horror, return all non-Horror
            //  creatures to their owners' hands.)
    )
}

fn on_instant_or_sorcery(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(ice_id) = reg.interner().lookup("ice") else { return Vec::new(); };
    vec![
        Effect::RemoveCounters {
            target: trig.source,
            kind: CounterKind::Named(ice_id),
            count: 1,
        },
        // GAP: "if no ice counters remain, transform" — counter-count
        // conditional cannot be expressed here; transform omitted.
    ]
}
