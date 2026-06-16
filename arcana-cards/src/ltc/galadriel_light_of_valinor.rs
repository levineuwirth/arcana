//! Galadriel, Light of Valinor — `{2}{G}{W}{U}` 3/3 Legendary Elf Noble.
//! Alliance — Whenever another creature you control enters, choose one
//! that hasn't been chosen this turn —
//! • Add {G}{G}{G}.
//! • Put a +1/+1 counter on each creature you control.
//! • Scry 2, then draw a card.
//!
//! Alliance/Scry are not usable `KeywordAbility` variants for this class —
//! keyword line omitted. The trigger condition (another creature you
//! control enters) is expressible, but the per-resolution modal choice
//! ("choose one that hasn't been chosen this turn") is not expressible on
//! a triggered ability (modal lives only on spell abilities). GAP the
//! effect body.

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
    let name = reg.interner_mut().intern("Galadriel, Light of Valinor");
    let elf = reg.interner_mut().intern("Elf");
    let noble = reg.interner_mut().intern("Noble");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(noble);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{W}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                from: None,
                to: Zone::Battlefield,
            },
            intervening_if: None,
            effect: alliance_modal,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn alliance_modal(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: per-resolution "choose one that hasn't been chosen this turn"
    // modal selection is not expressible on a triggered ability.
    Vec::new()
}
