//! Scaretiller — `{4}` Artifact Creature — Scarecrow, 1/4.
//! "Whenever this creature becomes tapped, choose one —
//!   • You may put a land card from your hand onto the battlefield tapped.
//!   • Return target land card from your graveyard to the battlefield tapped."
//!
//! Modal triggered abilities have no ModalSpec hook on TriggeredAbilityDef
//! (modal is a spell-ability mechanism). The first mode (put a land from hand
//! onto the battlefield tapped) is emitted faithfully and non-targeted; the
//! second mode (return target land from graveyard) is GAP'd, and the
//! "choose one" branch is therefore collapsed to mode 1.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Scaretiller");
    let scarecrow = reg.interner_mut().intern("Scarecrow");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(scarecrow);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfBecomesTapped,
            intervening_if: None,
            effect: tapped_put_land,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn tapped_put_land(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // Mode 1: "You may put a land card from your hand onto the battlefield tapped."
    // GAP (mode 2): "Return target land card from your graveyard to the
    // battlefield tapped" — no graveyard-land-to-battlefield-tapped target form.
    vec![Effect::PutFromHandOntoBattlefield {
        player: trig.controller,
        filter: ObjectFilter::permanent().with_types(TypeLine::LAND.into()),
        tapped: true,
    }]
}
