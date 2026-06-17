//! Timothar, Baron of Bats — `{4}{B}{B}` 4/4 Legendary Vampire Noble.
//!
//! * Ward—Discard a card. (Non-mana ward cost — NOT expressible; `KeywordAbility::Ward`
//!   takes a `ManaCost` only, so this is GAP'd and `keywords` is empty.)
//! * Whenever another nontoken Vampire you control dies, you may pay {1} and exile it.
//!   If you do, create a 1/1 black Bat creature token with flying that gains a
//!   sacrifice-and-return-the-exiled-card triggered ability.
//!
//! The "exile it … return the exiled card" linkage between the dying Vampire and the
//! Bat's combat-damage trigger is not expressible with the demonstrated API; we model
//! the payable gate and the Bat token creation, and GAP the exile/return rider.

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
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
    let name = reg.interner_mut().intern("Timothar, Baron of Bats");
    let vampire = reg.interner_mut().intern("Vampire");
    let noble = reg.interner_mut().intern("Noble");
    let _bat = reg.interner_mut().intern("Bat");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(noble);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        // GAP: Ward—Discard a card is a non-mana ward cost; KeywordAbility::Ward takes a ManaCost only.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: ObjectFilter::new()
                    .with_subtype_sym(vampire)
                    .controlled_by(ControllerConstraint::You)
                    .nontoken(),
                from: Some(Zone::Battlefield),
                to: Zone::Graveyard(0),
            },
            intervening_if: None,
            effect: timothar_vampire_dies,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn timothar_vampire_dies(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let bat = reg.interner().lookup("Bat").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bat);
    // You may pay {1}; if you do, create a 1/1 black flying Bat token.
    // GAP: "exile it [the dying Vampire]" and the Bat's "sacrifice it and return the
    // exiled card to the battlefield tapped" linkage are not expressible.
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{1}").expect("valid cost")),
        then: Box::new(Effect::CreateToken {
            controller: trig.controller,
            token: TokenDefinition {
                name: bat,
                colors: ColorSet::black(),
                types: TypeLine::CREATURE.into(),
                subtypes,
                power: Some(PtValue::Fixed(1)),
                toughness: Some(PtValue::Fixed(1)),
                keywords: vec![KeywordAbility::Flying],
                abilities: vec![],
            },
        }),
        else_effect: None,
    }]
}
