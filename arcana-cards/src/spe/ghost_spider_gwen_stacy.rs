//! Ghost-Spider, Gwen Stacy — `{3}{R}{R}` 4/4 Legendary Spider Human Hero with
//! Menace. Whenever Ghost-Spider attacks, she deals X damage to defending
//! player, where X is the number of attacking creatures.

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
    let name = reg.interner_mut().intern("Ghost-Spider, Gwen Stacy");
    let spider = reg.interner_mut().intern("Spider");
    let human = reg.interner_mut().intern("Human");
    let hero = reg.interner_mut().intern("Hero");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spider);
    subtypes.0.insert(human);
    subtypes.0.insert(hero);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: deal_x_to_defender,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn deal_x_to_defender(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "X = the number of ATTACKING creatures" — the documented script /
    // ObjectFilter surface has no attacking-only count, so X is not computable.
    // Per the dynamic-amount rule, the whole effect is omitted rather than emit a
    // wrong literal.
    Vec::new()
}
