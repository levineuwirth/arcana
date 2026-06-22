//! Spectral Force — `{3}{G}{G}` 8/8 green Elemental Spirit with Trample.
//!
//! * Trample — keyword.
//! * "Whenever this creature attacks, if defending player controls no black
//!   permanents, it doesn't untap during your next untap step." — the
//!   SelfAttacks trigger is faithful, but:
//!     - GAP (intervening-if): "if the defending player controls no black
//!       permanents" inspects the DEFENDING player's board; the available
//!       `conditions::` predicates only test the controller's own board, so
//!       this gate cannot be expressed.
//!     - GAP (effect): "doesn't untap during your next untap step" has no
//!       corresponding `Effect` variant.

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
    let name = reg.interner_mut().intern("Spectral Force");
    let elemental = reg.interner_mut().intern("Elemental");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(8)),
        toughness: Some(PtValue::Fixed(8)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: attacks_no_untap,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn attacks_no_untap(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "doesn't untap during your next untap step" — no Effect variant;
    // and the "defending player controls no black permanents" gate is also
    // inexpressible (see module doc).
    Vec::new()
}
