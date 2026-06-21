//! Callous Bloodmage — `{2}{B}` 2/1 Vampire Warlock.
//! "When this creature enters, choose one —
//!  • Create a 1/1 black and green Pest creature token with 'When this
//!    token dies, you gain 1 life.'
//!  • You draw a card and you lose 1 life.
//!  • Exile target player's graveyard."
//!
//! This is a MODAL triggered ability ("choose one"). The documented API
//! exposes modal only on spell abilities (ModalSpec/dispatch_modal_effect),
//! not on TriggeredAbilityDef, and there is no choose-a-mode Effect. The
//! ETB trigger is wired but its modal choice is a GAP — emitting any one
//! fixed mode would be materially wrong.

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
    let name = reg.interner_mut().intern("Callous Bloodmage");
    let vampire = reg.interner_mut().intern("Vampire");
    let warlock = reg.interner_mut().intern("Warlock");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(warlock);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_choose_one,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_choose_one(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: modal "choose one —" on a triggered ability. The usable API only
    // supports modal on spell abilities (ModalSpec), not on triggers, and
    // there is no choose-a-mode Effect. All three modes are individually
    // expressible, but the per-trigger mode selection is not — so the whole
    // modal payload is GAP'd rather than locking in one fixed mode.
    Vec::new()
}
