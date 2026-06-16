//! Megatherium — `{2}{G}` 4/4 Beast with Trample. "When this creature
//! enters, sacrifice it unless you pay {1} for each card in your hand."
//! The "unless you pay" gate's punishment is sacrificing THIS specific
//! permanent, for which there is no expressible immediate
//! sacrifice-source effect (Effect::Sacrifice chooses among a filter,
//! not the source itself), so the trigger body is gapped.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Megatherium");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: enter_tax,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn enter_tax(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "sacrifice it unless you pay {1} for each card in your hand"
    // — the punishment is sacrificing THIS source permanent, and there
    // is no immediate sacrifice-source effect (Effect::Sacrifice picks
    // among a filter, not the source). Modeling it as a generic
    // sacrifice would be materially wrong, so omitted.
    Vec::new()
}
