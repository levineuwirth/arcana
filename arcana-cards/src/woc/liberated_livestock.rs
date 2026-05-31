//! Liberated Livestock — `{5}{W}` 4/6 white Cat Bird Ox. "When this
//! creature dies, create a 1/1 white Cat creature token with lifelink,
//! a 1/1 white Bird creature token with flying, and a 2/4 white Ox
//! creature token. For each of those tokens, you may put an Aura card
//! from your hand and/or graveyard onto the battlefield attached to it."
//!
//! The three tokens are minted faithfully. The optional "put an Aura
//! card from your hand and/or graveyard onto the battlefield attached
//! to it" rider is not expressible — see GAP in the effect fn.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
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
    let name = reg.interner_mut().intern("Liberated Livestock");
    let cat = reg.interner_mut().intern("Cat");
    let bird = reg.interner_mut().intern("Bird");
    let ox = reg.interner_mut().intern("Ox");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);
    subtypes.0.insert(bird);
    subtypes.0.insert(ox);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: dies_create_tokens,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn dies_create_tokens(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let cat = reg.interner().lookup("Cat").expect("Cat interned during register()");
    let bird = reg.interner().lookup("Bird").expect("Bird interned during register()");
    let ox = reg.interner().lookup("Ox").expect("Ox interned during register()");

    let mut cat_subtypes = SubtypeSet::default();
    cat_subtypes.0.insert(cat);
    let cat_token = TokenDefinition {
        name: cat,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes: cat_subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Lifelink],
        abilities: vec![],
    };

    let mut bird_subtypes = SubtypeSet::default();
    bird_subtypes.0.insert(bird);
    let bird_token = TokenDefinition {
        name: bird,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes: bird_subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        abilities: vec![],
    };

    let mut ox_subtypes = SubtypeSet::default();
    ox_subtypes.0.insert(ox);
    let ox_token = TokenDefinition {
        name: ox,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes: ox_subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![],
        abilities: vec![],
    };

    // GAP: "For each of those tokens, you may put an Aura card from your
    // hand and/or graveyard onto the battlefield attached to it" — there
    // is no effect variant for putting an Aura from hand/graveyard onto
    // the battlefield attached to a freshly-created token. Tokens minted
    // faithfully; the optional Aura-attachment rider is omitted.
    vec![
        Effect::CreateToken { controller: trig.controller, token: cat_token },
        Effect::CreateToken { controller: trig.controller, token: bird_token },
        Effect::CreateToken { controller: trig.controller, token: ox_token },
    ]
}
