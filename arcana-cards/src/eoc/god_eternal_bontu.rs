//! God-Eternal Bontu — `{3}{B}{B}` 5/6 Legendary Zombie God with Menace.
//!
//! Oracle:
//! * Menace.
//! * When God-Eternal Bontu enters, sacrifice any number of other
//!   permanents, then draw that many cards. — the "sacrifice any number
//!   of other permanents" half is expressed via ChooseAnyNumberFromZone
//!   (Sacrifice); the "then draw that many cards" rider's count is the
//!   number sacrificed, which cannot be linked to the variable pick → GAP'd.
//! * When God-Eternal Bontu dies or is put into exile from the
//!   battlefield, you may put it into its owner's library third from the
//!   top. — "third from the top" library placement is not an expressible
//!   Effect → GAP'd.

use arcana_core::effects::{Effect, KeywordAbility, PickAction};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("God-Eternal Bontu");
    let zombie = reg.interner_mut().intern("Zombie");
    let god = reg.interner_mut().intern("God");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(god);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_sacrifice_any_number,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: on_dies,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_sacrifice_any_number(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "then draw that many cards" — the draw count equals the number
    // of permanents sacrificed by the ChooseAnyNumberFromZone pick, which
    // cannot be linked back to the variable count. Only the sacrifice is
    // expressed (any number of OTHER permanents you control).
    vec![Effect::ChooseAnyNumberFromZone {
        chooser: trig.controller,
        zone: Zone::Battlefield,
        filter: ObjectFilter::permanent().controlled_by(ControllerConstraint::You),
        action: PickAction::Sacrifice,
    }]
}

fn on_dies(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "you may put it into its owner's library third from the top" —
    // there is no Effect for placing a card at a specific library depth.
    Vec::new()
}
