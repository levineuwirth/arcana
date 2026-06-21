//! Altaïr Ibn-La'Ahad — `{R}{W}{B}` 3/3 Legendary Human Assassin with First strike.
//! Whenever Altaïr attacks, exile up to one target Assassin creature card from
//! your graveyard with a memory counter on it. Then for each creature card you
//! own in exile with a memory counter on it, create a tapped and attacking token
//! that's a copy of it. Exile those tokens at end of combat.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Altaïr Ibn-La'Ahad");
    let human = reg.interner_mut().intern("Human");
    let assassin = reg.interner_mut().intern("Assassin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(assassin);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{W}{B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::FirstStrike],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: attack_memory_tokens,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn attack_memory_tokens(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: this ability requires memory-counter tracking on cards in exile, a
    // per-exiled-card "create a tapped & attacking token copy of it", and an
    // end-of-combat exile of those tokens — none expressible with the available
    // primitives. The whole effect is omitted.
    Vec::new()
}
