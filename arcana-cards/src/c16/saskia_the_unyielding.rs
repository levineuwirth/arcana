//! Saskia the Unyielding — `{B}{R}{G}{W}` 3/4 Legendary Human Soldier
//! with Vigilance and Haste.
//! "As Saskia enters, choose a player."
//! "Whenever a creature you control deals combat damage to a player,
//!  it deals that much damage to the chosen player."
//!
//! Vigilance and Haste are base keywords. The combat-damage trigger is
//! wired as a `DamageDealt` watcher over your creatures hitting a
//! player; its payload depends on the "chosen player" recorded by the
//! "As ~ enters" replacement choice, which has no expressible storage
//! in the demonstrated API — so both the as-enters choice and the
//! redirected-damage effect body are GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Saskia the Unyielding");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    // GAP: "As Saskia enters, choose a player." — an as-enters replacement
    //      choice recording a "chosen player" with no expressible storage
    //      in the demonstrated API.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{R}{G}{W}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red() | ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Vigilance, KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: deal_to_chosen_player,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn deal_to_chosen_player(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "it deals that much damage to the chosen player" — the chosen
    //      player comes from the GAP'd "As ~ enters, choose a player"
    //      replacement choice, which has no expressible storage; without it
    //      the redirect target is unknown.
    Vec::new()
}
