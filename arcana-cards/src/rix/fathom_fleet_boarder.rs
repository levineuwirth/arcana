//! Fathom Fleet Boarder — `{2}{B}` 3/3 Orc Pirate.
//! "When this creature enters, you lose 2 life unless you control
//! another Pirate."
//!
//! GAP: "unless you control another Pirate" conditional — no
//! Conditional effect checking a board state; emitting the life loss
//! unconditionally as best effort.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fathom Fleet Boarder");
    let orc = reg.interner_mut().intern("Orc");
    let pirate = reg.interner_mut().intern("Pirate");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(orc);
    subtypes.0.insert(pirate);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_lose_life_unless_pirate,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_lose_life_unless_pirate(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let pirate_filter = arcana_core::script::subtype_filter(reg, "Pirate")
        .controlled_by(arcana_core::targets::ControllerConstraint::You);
    let pirates = arcana_core::script::count_matching(state, &pirate_filter, trig.controller);
    // We control at least 1 pirate (self); if >1 then we have another pirate
    if pirates > 1 {
        Vec::new()
    } else {
        vec![Effect::LoseLife { player: trig.controller, amount: 2 }]
    }
}
