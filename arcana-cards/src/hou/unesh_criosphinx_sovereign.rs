//! Unesh, Criosphinx Sovereign — `{4}{U}{U}` 4/4 Legendary Sphinx.
//! "Flying.
//!  Sphinx spells you cast cost {2} less to cast.
//!  Whenever Unesh or another Sphinx you control enters, reveal the top
//!  four cards of your library. An opponent separates those cards into
//!  two piles. Put one pile into your hand and the other into your
//!  graveyard."
//!
//! Flying is a keyword. The cost-reduction static is not expressible and
//! is GAP'd. The Sphinx-enters trigger condition is wired, but its effect
//! (reveal four, opponent separates into two piles, you take one) has no
//! corresponding Effect primitive, so the effect body is GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

// GAP: static — "Sphinx spells you cast cost {2} less to cast" is a
// cost-reduction continuous ability, not a triggered/activated ability.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Unesh, Criosphinx Sovereign");
    let sphinx = reg.interner_mut().intern("Sphinx");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sphinx);

    let sphinx_filter =
        script::subtype_filter(reg, "Sphinx").controlled_by(ControllerConstraint::You);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: sphinx_filter,
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: sphinx_enters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn sphinx_enters(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: effect — "reveal the top four cards of your library. An opponent
    // separates those cards into two piles. Put one pile into your hand and
    // the other into your graveyard." No pile-separation Effect primitive
    // exists.
    Vec::new()
}
