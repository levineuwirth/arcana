//! Vrondiss, Rage of Ancients — `{3}{R}{G}` 5/4 Legendary Dragon Barbarian.
//! "Enrage — Whenever Vrondiss is dealt damage, you may create a 5/4 red
//!  and green Dragon Spirit creature token with 'When this token deals
//!  damage, sacrifice it.'"
//! "Whenever you roll one or more dice, you may have Vrondiss deal 1
//!  damage to itself." (dice-roll trigger — GAP)
//!
//! The Enrage trigger fires when Vrondiss is dealt damage and creates the
//! Dragon Spirit token. The token's embedded "When this token deals
//! damage, sacrifice it" ability is GAP'd (no expressible self-sacrifice
//! triggered ability on a token, matching existing Dragon-Spirit tokens).

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vrondiss, Rage of Ancients");
    let dragon = reg.interner_mut().intern("Dragon");
    let barbarian = reg.interner_mut().intern("Barbarian");
    let _spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);
    subtypes.0.insert(barbarian);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{G}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP (trigger): "Whenever you roll one or more dice, you may have
    // Vrondiss deal 1 damage to itself." — no dice-roll TriggerCondition.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfIsDealtDamage { combat_only: false },
                intervening_if: None,
                effect: enrage_make_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn enrage_make_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let dragon = reg.interner().lookup("Dragon").unwrap_or_default();
    let spirit = reg.interner().lookup("Spirit").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);
    subtypes.0.insert(spirit);
    // GAP (token ability): "When this token deals damage, sacrifice it." —
    // no expressible self-sacrifice triggered ability on a token.
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: dragon,
            colors: ColorSet::red() | ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(5)),
            toughness: Some(PtValue::Fixed(4)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
