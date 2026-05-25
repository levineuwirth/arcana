//! Emperor Apatzec Intli IV — `{R}{G}{W}` 3/4 legendary red-green-white Human Noble.
//! "Whenever another creature enters under your control, that creature
//! perpetually gains haste if its power is 4 or greater. If its
//! toughness is 4 or greater, you gain 4 life. If its mana value is
//! 4 or greater, seek a creature card."
//! GAP: "perpetually gains haste" (persistent keyword grant) not in
//! engine catalog; "seek" not in engine catalog. Life gain emitted
//! unconditionally (the power/toughness/CMC conditionals require
//! accessing entering object stats which needs entering_object()).
//! Keywords: Seek not supported — omitted.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Emperor Apatzec Intli IV");
    let human = reg.interner_mut().intern("Human");
    let noble = reg.interner_mut().intern("Noble");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(noble);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{G}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: creature_etb_rider,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn creature_etb_rider(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "perpetually gains haste if power >= 4" — no perpetual keyword grant effect
    // GAP: "if toughness >= 4, gain 4 life" — conditional on entering creature's stats
    // GAP: "if mana value >= 4, seek a creature card" — Seek not in engine catalog
    // Emitting unconditional life gain as partial approximation
    vec![Effect::GainLife { player: trig.controller, amount: 4 }]
}
