//! Momo, Playful Pet — `{W}` 1/1 Legendary Lemur Bat Ally (W).
//!
//! * Flying, vigilance.
//! * "When Momo leaves the battlefield, choose one — Create a Food
//!   token; or Put a +1/+1 counter on target creature you control; or
//!   Scry 2."
//!
//! GAP: the modal choice on the leaves-the-battlefield TRIGGER. Modal
//! dispatch (`ModalSpec` + `with_mode_effects`) is a SPELL-ability
//! primitive; the demonstrated triggered-ability API has no per-mode
//! callback surface, and a targeted mode (the +1/+1 counter) shares the
//! same trigger as untargeted modes (Food / Scry), which can't be
//! expressed without modal target concatenation. The
//! `SelfLeavesBattlefield` trigger is wired with a GAP'd effect so the
//! bones + keywords land; emitting one arbitrary mode would be a
//! materially wrong card.

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
    let name = reg.interner_mut().intern("Momo, Playful Pet");
    let lemur = reg.interner_mut().intern("Lemur");
    let bat = reg.interner_mut().intern("Bat");
    let ally = reg.interner_mut().intern("Ally");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lemur);
    subtypes.0.insert(bat);
    subtypes.0.insert(ally);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfLeavesBattlefield,
                intervening_if: None,
                effect: leaves_modal,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn leaves_modal(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "choose one — Food token / +1/+1 counter on target / Scry 2".
    // Modal dispatch is a spell-only primitive; no per-mode callback for
    // triggered abilities, and one mode targets while two don't.
    Vec::new()
}
