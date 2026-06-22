//! Charmed Griffin — `{3}{W}` 3/3 Griffin with Flying.
//! "When this creature enters, each other player may put an artifact or
//! enchantment card onto the battlefield from their hand."
//!
//! Decomposition:
//! * Flying — keyword.
//! * ETB group-hug — `SelfEntersBattlefield` trigger; for each opponent, post a
//!   "put an artifact or enchantment card from your hand onto the battlefield"
//!   pick. ("may" is the inherent min-0 nature of the hand pick.)

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Charmed Griffin");
    let griffin = reg.interner_mut().intern("Griffin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(griffin);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_each_other_player_put,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_each_other_player_put(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let opponents = script::opponents(state, trig.controller);
    let effects: Vec<Effect> = opponents
        .into_iter()
        .map(|p| Effect::PutFromHandOntoBattlefield {
            player: p,
            filter: ObjectFilter::new()
                .with_types_any(TypeLine(TypeLine::ARTIFACT | TypeLine::ENCHANTMENT)),
            tapped: false,
        })
        .collect();
    vec![Effect::Sequence(effects)]
}
