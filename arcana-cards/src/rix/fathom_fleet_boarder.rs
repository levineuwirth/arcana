//! Fathom Fleet Boarder — `{2}{B}` 3/3 Orc Pirate. "When this creature
//! enters, you lose 2 life unless you control another Pirate."
//!
//! GAP: intervening-if "unless you control another Pirate" — emitted as
//! None; the conditional life loss cannot be expressed as a pure Effect
//! with the available catalog.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
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
    let _pirate_filter = reg.interner_mut().intern("Pirate");
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
    let pirate_filter = script::subtype_filter(reg, "Pirate");
    let pirate_count = script::count_matching(state, &pirate_filter, trig.controller);
    // pirate_count includes self; "another Pirate" = count > 1
    if pirate_count <= 1 {
        vec![Effect::LoseLife { player: trig.controller, amount: 2 }]
    } else {
        Vec::new()
    }
}
