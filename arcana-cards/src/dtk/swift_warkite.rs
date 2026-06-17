//! Swift Warkite — `{4}{B}{R}` 4/4 Dragon with Flying.
//!
//! Flying.
//! When this creature enters, you may put a creature card with mana value 3
//! or less from your hand or graveyard onto the battlefield. That creature
//! gains haste. Return it to your hand at the beginning of the next end step.
//!
//! The ETB put-onto-battlefield (from hand) is modeled with
//! `Effect::PutFromHandOntoBattlefield`; the haste grant and the delayed
//! return are GAP'd because the chosen object's id isn't available to the
//! resolver to wire the rider effects.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Swift Warkite");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_put_creature,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_put_creature(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "you may put a creature card with mana value 3 or less from your hand
    // onto the battlefield." Graveyard half + the haste grant + the
    // delayed end-step return are not expressible here (resolver doesn't see
    // the chosen object's id to attach riders).
    // GAP: graveyard source, haste grant, and delayed return-to-hand riders
    vec![Effect::PutFromHandOntoBattlefield {
        player: trig.controller,
        filter: ObjectFilter::creature().with_max_cmc(3),
        tapped: false,
    }]
}
