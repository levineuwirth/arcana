//! Citanul Woodreaders — `{2}{G}` 1/4 Human Druid with Kicker {2}{G}.
//! "Kicker {2}{G}
//!  When this creature enters, if it was kicked, draw two cards."
//!
//! Kicker is not in the usable KeywordAbility surface — GAP'd (keywords
//! left empty). The ETB "if it was kicked, draw two cards" is wired as a
//! trigger, but the "was kicked" gate is not expressible (no kicked-
//! status predicate/accessor), so the effect is GAP'd rather than draw
//! unconditionally.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Citanul Woodreaders");
    let human = reg.interner_mut().intern("Human");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(druid);

    // GAP (keyword): "Kicker {2}{G}" — Kicker is not in the usable
    // KeywordAbility surface; keywords left empty.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_if_kicked_draw_two,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_if_kicked_draw_two(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "if it was kicked, draw two cards" — no kicked-status predicate or
    // accessor is available, so the kicked gate can't be evaluated. Emitting
    // an unconditional draw would be materially wrong; GAP the whole effect.
    Vec::new()
}
