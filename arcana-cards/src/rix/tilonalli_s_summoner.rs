//! Tilonalli's Summoner — `{1}{R}` 1/1 red Human Shaman.
//! "Ascend. Whenever this creature attacks, you may pay {X}{R}. If you do,
//! create X 1/1 red Elemental creature tokens that are tapped and attacking.
//! At the beginning of the next end step, exile those tokens unless you have
//! the city's blessing."

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
    let name = reg.interner_mut().intern("Tilonalli's Summoner");
    let human = reg.interner_mut().intern("Human");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(shaman);
    // GAP: "Ascend" — not in the supported KeywordAbility set; the city's
    // blessing state is not modeled.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: pay_x_make_elementals,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn pay_x_make_elementals(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may pay {X}{R}. If you do, create X 1/1 ... tapped and
    // attacking ... exile those tokens unless city's blessing" —
    // OptionalPaymentKind only supports a FIXED Mana/Life cost (no variable
    // X), and the X-many token count plus the city's-blessing conditional
    // exile rider are not expressible.
    Vec::new()
}
