//! Sylvia Brightspear — `{2}{W}` 2/2 Legendary Human Knight with Double strike.
//!
//! Oracle:
//! * "Partner with Khorvath Brightflame (When this creature enters, target
//!    player may put Khorvath into their hand from their library, then shuffle.)"
//!    — Partner / Partner with are not in the usable keyword surface; the ETB
//!    (target player may tutor a named card to their OWN hand) has no effect for
//!    a chosen player searching their own library; GAP'd.
//! * Double strike — keyword, base characteristic.
//! * "Dragons your team controls have double strike." — GAP: a static
//!   continuous anthem (no trigger word, no cost); not expressible.

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
    let name = reg.interner_mut().intern("Sylvia Brightspear");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::DoubleStrike],
        ..Default::default()
    };

    // GAP: static "Dragons your team controls have double strike."
    reg.register(
        CardDefinition::new(name, chars)
            // GAP: Partner-with ETB "target player may put Khorvath into their
            //      hand from their library" — no chosen-player self-tutor effect.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_partner_with,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_partner_with(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: target player may tutor a named card to their own hand — no effect
    //      for a chosen player searching their own library.
    Vec::new()
}
