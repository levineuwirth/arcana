//! The Master of Keys — `{X}{W}{U}{B}` 3/3 Legendary Enchantment Creature — Horror with Flying.
//! When it enters, put X +1/+1 counters on it and mill twice X cards.
//! Each enchantment card in your graveyard has escape (escape cost = mana cost + exile
//! three other cards from your graveyard). (static — GAP'd)
//!
//! The ETB uses the spell's cast X, which no enters-battlefield trigger accessor exposes,
//! so its counters-and-mill effect is GAP'd. The graveyard-wide "has escape" grant is a
//! static with no expressible primitive — GAP'd. ("Mill" is not a real keyword ability.)

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
    let name = reg.interner_mut().intern("The Master of Keys");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(horror);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{X}{W}{U}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue() | ColorSet::black(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: "Each enchantment card in your graveyard has escape …" — a graveyard-wide
    //      ability grant; not expressible.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_counters_and_mill,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_counters_and_mill(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "put X +1/+1 counters on it and mill twice X cards" — the cast X is not
    //      exposed on an enters-battlefield trigger, so the dynamic count cannot be computed.
    Vec::new()
}
