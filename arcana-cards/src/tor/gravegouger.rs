//! Gravegouger — `{2}{B}` 2/2 Nightmare Horror.
//! When this creature enters, exile up to two target cards from a
//! single graveyard.
//! When this creature leaves the battlefield, return the exiled cards
//! to their owner's graveyard.

use arcana_core::effects::{Effect, PickAction};
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
    let name = reg.interner_mut().intern("Gravegouger");
    let nightmare = reg.interner_mut().intern("Nightmare");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(nightmare);
    subtypes.0.insert(horror);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_exile_from_graveyard,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfLeavesBattlefield,
                intervening_if: None,
                effect: leaves_return_exiled,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_exile_from_graveyard(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Fidelity gap: "from a single graveyard" — ChooseNFromZone reads
    // one fixed zone, so we pick up to two cards from an opponent's
    // graveyard (the common case).
    let opps = script::opponents(state, trig.controller);
    let Some(opp) = opps.first() else {
        return Vec::new();
    };
    vec![Effect::ChooseNFromZone {
        chooser: trig.controller,
        zone: Zone::Graveyard(*opp),
        filter: ObjectFilter::default(),
        min: 0,
        max: 2,
        action: PickAction::Exile,
    }]
}

fn leaves_return_exiled(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "return the exiled cards to their owner's graveyard" — no
    // primitive returns the specific set of cards exiled by an earlier
    // ETB resolution (no exile-linkage to graveyard).
    Vec::new()
}
