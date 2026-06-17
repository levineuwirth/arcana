//! Champion of Wits — `{2}{U}` 2/1 blue Snake Wizard.
//! When this creature enters, you may draw cards equal to its power. If you do,
//! discard two cards.
//! GAP: Eternalize is not in the available keyword surface and its graveyard
//! copy-token activation is not modeled — omitted.
//! GAP: the "you may" optionality on the ETB draw isn't expressible without a
//! payment gate; implemented as the mandatory draw-then-discard core.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Champion of Wits");
    let snake = reg.interner_mut().intern("Snake");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(snake);
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_loot,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_loot(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let n = script::power_of(state, trig.source).max(0) as u32;
    vec![Effect::Sequence(vec![
        Effect::DrawCards { player: trig.controller, count: n },
        Effect::Discard {
            player: trig.controller,
            count: 2,
            choice: DiscardChoice::ControllerChooses,
        },
    ])]
}
