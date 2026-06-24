//! Acererak the Archlich — `{2}{B}` 5/5 Legendary Zombie Wizard.
//! When Acererak enters, if you haven't completed Tomb of Annihilation,
//! return it to hand and venture into the dungeon.
//! Whenever Acererak attacks, for each opponent, you create a 2/2 black
//! Zombie token unless that player sacrifices a creature.

use arcana_core::actions::{OptionalPaymentKind, SacrificeFilter};
use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Acererak the Archlich");
    let zombie = reg.interner_mut().intern("Zombie");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            // ETB: the "if you haven't completed Tomb of Annihilation"
            // intervening-if and the "return to its owner's hand" rider are
            // not expressible (no dungeon-completion predicate, and bouncing
            // self is not a documented effect); only the venture is emitted.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_venture,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Attack: "for each opponent, create a 2/2 black Zombie unless
            // that player sacrifices a creature of their choice." Per-opponent
            // OptionalPayment whose chooser is that opponent (pay = sacrifice a
            // creature, decline = you create the Zombie).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_zombies,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_venture(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Venture { player: trig.controller }]
}

fn attack_zombies(state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let zombie = reg.interner().lookup("Zombie").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    let token = TokenDefinition {
        name: zombie,
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        abilities: vec![],
    };
    script::opponents(state, trig.controller)
        .into_iter()
        .map(|p| Effect::OptionalPayment {
            chooser: p,
            cost: OptionalPaymentKind::Sacrifice(SacrificeFilter::Creature),
            then: Box::new(Effect::Sequence(vec![])),
            else_effect: Some(Box::new(Effect::CreateToken {
                controller: trig.controller,
                token: token.clone(),
            })),
        })
        .collect()
}
