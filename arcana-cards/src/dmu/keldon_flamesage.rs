//! Keldon Flamesage — `{2}{R}` 2/3 Human Shaman with Enlist.
//!
//! * Enlist.
//! * Whenever this creature attacks, look at the top X cards of your library,
//!   where X is this creature's power. You may exile an instant or sorcery
//!   card with mana value X or less from among them. Put the rest on the
//!   bottom of your library in a random order. You may cast the exiled card
//!   without paying its mana cost.
//!
//! Modeled with `DigTopN` (look at top X = power, optionally take one
//! instant/sorcery, rest to the bottom). FIDELITY GAP: the chosen card goes
//! to hand rather than being free-cast, and the `mana value X or less`
//! restriction is not applied (only the instant/sorcery type filter is).

use arcana_core::effects::{DigRest, Effect};
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
    let name = reg.interner_mut().intern("Keldon Flamesage");
    let human = reg.interner_mut().intern("Human");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(shaman);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![arcana_core::effects::KeywordAbility::Enlist],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: attack_dig,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn attack_dig(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let x = script::power_of(state, trig.source).max(0) as u32;
    if x == 0 {
        return Vec::new();
    }
    let filter = ObjectFilter::new()
        .with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY));
    vec![Effect::DigTopN {
        player: trig.controller,
        count: x,
        filter: Some(filter),
        rest: DigRest::BottomRandom,
    }]
}
