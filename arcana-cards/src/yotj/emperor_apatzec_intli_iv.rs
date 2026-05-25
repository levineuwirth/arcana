//! Emperor Apatzec Intli IV — `{R}{G}{W}` 3/4 legendary red-green-white
//! Human Noble.
//! "Whenever another creature enters under your control, that creature
//! perpetually gains haste if its power is 4 or greater. If its toughness
//! is 4 or greater, you gain 4 life. If its mana value is 4 or greater,
//! seek a creature card."
//! GAP: "perpetually gains haste" — no catalog Effect variant for perpetual
//! keyword grants.
//! GAP: "seek a creature card" — TutorToHand with reveal:false approximates
//! seek; power/toughness/MV conditionals cannot be evaluated at resolve
//! time without more script helpers. Using GainLife for the toughness≥4
//! branch (unconditional — GAP). Other branches return Vec::new().

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
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: creature_enters_conditionals,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn creature_enters_conditionals(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "perpetually gains haste if power ≥ 4" — no perpetual keyword
    // grant in catalog.
    // GAP: conditional on toughness ≥ 4 and MV ≥ 4 not evaluable without
    // accessing the triggering object's stats; implementing unconditional
    // best-effort: gain 4 life (toughness branch), seek omitted.
    vec![Effect::GainLife { player: trig.controller, amount: 4 }]
}
