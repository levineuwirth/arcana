//! Scion of Vitu-Ghazi — `{3}{W}{W}` 4/4 white Elemental creature.
//! "When this creature enters, if you cast it from your hand, create
//! a 1/1 white Bird creature token with flying, then populate."
//!
//! GAP: intervening_if — "if you cast it from your hand" cannot be
//! expressed with the current `TriggeredAbilityDef.intervening_if`
//! surface (no cast-from-hand predicate); trigger fires
//! unconditionally on ETB.
//! GAP: populate — no `Effect::Populate` variant in the catalog; the
//! "then populate" clause is not emitted. The Bird token creation
//! IS emitted.
//! GAP: Populate listed as a Scryfall keyword on the card is an
//! ability-word / one-shot, not in the supported keyword surface —
//! `keywords: vec![]`.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
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
    let name = reg.interner_mut().intern("Scion of Vitu-Ghazi");
    let elemental = reg.interner_mut().intern("Elemental");
    // Pre-intern the token subtype so the trigger's resolver can
    // look it up via the non-mut interner at resolve time.
    let _bird = reg.interner_mut().intern("Bird");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_create_bird_then_populate,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB trigger resolution: create a 1/1 white Bird creature token
/// with flying. The "then populate" clause is a GAP — no
/// `Effect::Populate` variant exists in the catalog.
fn etb_create_bird_then_populate(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let bird = reg
        .interner()
        .lookup("Bird")
        .expect("Bird interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    let token = TokenDefinition {
        name: bird,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        abilities: vec![],
    };
    // GAP: populate — no `Effect::Populate` variant available to
    // emit "create a token that's a copy of a creature token you
    // control" as a follow-on effect.
    vec![Effect::CreateToken {
        controller: trig.controller,
        token,
    }]
}
