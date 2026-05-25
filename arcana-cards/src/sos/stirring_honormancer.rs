//! Stirring Honormancer — `{2}{W}{W/B}{B}` 4/5 Rhino Bard. "When this
//! creature enters, look at the top X cards of your library, where X is the
//! number of creatures you control. Put one of those cards into your hand and
//! the rest into your graveyard."
//!
//! GAP: look-top-X-put-one-rest-to-graveyard is not in the Effect catalog
//! (only Scry/Surveil); emitting Surveil N as best effort where N = creature count.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Stirring Honormancer");
    let rhino = reg.interner_mut().intern("Rhino");
    let bard = reg.interner_mut().intern("Bard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rhino);
    subtypes.0.insert(bard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W/B}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: on_etb,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_etb(
    state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    let n = script::count_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    if n == 0 {
        return Vec::new();
    }
    // GAP: look-N-put-one-rest-to-graveyard not in Effect catalog;
    // emitting Surveil N as best effort.
    vec![Effect::Surveil { player: trig.controller, count: n }]
}
