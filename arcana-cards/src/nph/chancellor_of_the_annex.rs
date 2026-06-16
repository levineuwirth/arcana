//! Chancellor of the Annex — `{4}{W}{W}{W}` 5/6 white Phyrexian Angel
//! with Flying.
//! "You may reveal this card from your opening hand. If you do, when
//!  each opponent casts their first spell of the game, counter that
//!  spell unless that player pays {1}."
//! "Whenever an opponent casts a spell, counter it unless that player
//!  pays {1}."
//!
//! Flying is a base keyword. The battlefield "tax" trigger fires on an
//! opponent casting a spell; its counter-unless-pays payload is not in
//! the demonstrated effect catalog, so the effect body is GAP'd while
//! the trigger shape is preserved. The opening-hand reveal clause is a
//! pre-game mechanic and is not expressible here.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chancellor of the Annex");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let angel = reg.interner_mut().intern("Angel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(angel);

    // GAP: "You may reveal this card from your opening hand …" — opening-hand
    //      pre-game reveal mechanic; not expressible with the demonstrated API.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::Opponent,
                },
                intervening_if: None,
                effect: counter_unless_pays,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn counter_unless_pays(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "counter it unless that player pays {1}" — no counter /
    //      counter-unless-pays Effect variant in the demonstrated catalog.
    Vec::new()
}
