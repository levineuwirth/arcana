//! Ruin Processor — `{7}` 7/8 colorless Eldrazi Processor.
//! "When you cast this spell, you may put a card an opponent owns from exile into
//! that player's graveyard. If you do, you gain 5 life."
//! GAP: "SpellCast self" trigger not in catalog; using SelfEntersBattlefield as
//! best-effort (fires on ETB rather than on cast).
//! GAP: Moving a card an opponent owns from exile to their graveyard is not in the
//! engine effect catalog; effect returns GainLife 5 as partial best-effort.

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
    let name = reg.interner_mut().intern("Ruin Processor");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let processor = reg.interner_mut().intern("Processor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);
    subtypes.0.insert(processor);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{7}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(8)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: "when you cast this spell" trigger not in catalog; using ETB as closest
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: on_cast_gain_life,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_cast_gain_life(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Moving an opponent's exiled card to their graveyard not in engine effect catalog.
    // Emitting the conditional GainLife 5 as best-effort for the "if you do" branch.
    vec![Effect::GainLife { player: trig.controller, amount: 5 }]
}
