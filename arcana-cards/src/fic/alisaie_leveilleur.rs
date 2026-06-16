//! Alisaie Leveilleur — `{2}{W}` 3/2 Legendary Elf Wizard with First strike.
//! "Partner with Alphinaud Leveilleur (When this creature enters, target player
//! may put Alphinaud Leveilleur into their hand from their library, then
//! shuffle.) First strike. Dualcast — The second spell you cast each turn costs
//! {2} less to cast."
//!
//! First strike is a base keyword. Partner / Partner with are not in the usable
//! keyword surface. The "Partner with" ETB (target player may tutor a named
//! card to their own hand) has no demonstrated effect for a chosen player
//! tutoring from their own library, so it is GAP'd. Dualcast is a static cost
//! reduction with no API surface — GAP'd.

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
    let name = reg.interner_mut().intern("Alisaie Leveilleur");
    let elf = reg.interner_mut().intern("Elf");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::FirstStrike],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // GAP: Partner-with ETB "target player may put Alphinaud into their
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
        // GAP static: "Dualcast — The second spell you cast each turn costs {2}
        //      less to cast." — cost reduction with no API surface.
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
