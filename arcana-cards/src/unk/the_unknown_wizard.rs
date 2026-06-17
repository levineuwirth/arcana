//! The Unknown Wizard — `{2}{W}{U}{B}{R}{G}` 6/6 Legendary Human Wizard.
//! Eminence — other playtest cards you cast cost {1} less (static: GAP).
//! Whenever this enters or attacks, look at the top ten cards, you may put a
//! legendary playtest card onto the battlefield, rest on bottom (dig-to-
//! battlefield with a playtest-card filter: GAP).

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
    let name = reg.interner_mut().intern("The Unknown Wizard");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    // GAP: Eminence cost-reduction static (command-zone-or-battlefield) is not
    //      an expressible ability here.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{U}{B}{R}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green() | ColorSet::red() | ColorSet::blue() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: dig_for_legend,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: dig_for_legend,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn dig_for_legend(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "look at top ten, put a legendary PLAYTEST card onto the
    // battlefield" — no dig-to-battlefield variant and no playtest-card filter.
    Vec::new()
}
