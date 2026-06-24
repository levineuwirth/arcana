//! Swarm Culler — `{3}{B}` 2/4 black Insect Warrior with Flying.
//!
//! * Flying.
//! * `Whenever this creature becomes tapped, you may sacrifice another
//!   creature or artifact. If you do, draw a card.` — wired via an optional
//!   sacrifice payment: sacrifice a creature or artifact, then draw a card.
//!   Minor over-inclusion: the selection can't exclude the source, so Swarm
//!   Culler itself is technically offerable ("another"); harmless in practice.

use arcana_core::actions::{OptionalPaymentKind, SacrificeFilter};
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
    let name = reg.interner_mut().intern("Swarm Culler");
    let insect = reg.interner_mut().intern("Insect");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(insect);
    subtypes.0.insert(warrior);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfBecomesTapped,
                intervening_if: None,
                effect: may_sac_then_draw,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn may_sac_then_draw(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "you may sacrifice another creature or artifact; if you do, draw a card."
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Sacrifice(SacrificeFilter::CreatureOrArtifact),
        then: Box::new(Effect::DrawCards { player: trig.controller, count: 1 }),
        else_effect: None,
    }]
}
