//! Cartographer's Companion — `{3}` colorless 2/1 Artifact Creature
//! — Gnome. "When this creature enters, create a Map token." The
//! Map token is a colorless artifact with `{1}, {T}, Sacrifice this
//! token: Target creature you control explores. Activate only as a
//! sorcery.` — recognised by subtype; the Explore activation itself
//! is deferred engine work (no `Explore` Effect variant), so the
//! token is emitted cleanly without the activated ability wired up.

use arcana_core::effects::{Effect, TokenDefinition};
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
    let name = reg.interner_mut().intern("Cartographer's Companion");
    let gnome = reg.interner_mut().intern("Gnome");
    // Pre-intern the Map token's subtype so the resolver can look it
    // up via the non-mut interner at trigger-resolution time.
    let _map = reg.interner_mut().intern("Map");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(gnome);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_create_map_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB trigger: the ability's controller creates one Map artifact
/// token. The token's tap+sac → Explore activated ability is
/// recognised by the Map subtype but its activation is deferred
/// engine work (no `Effect::Explore` variant in catalog), so the
/// token body is emitted with no abilities attached.
fn etb_create_map_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let map = reg
        .interner()
        .lookup("Map")
        .expect("Map interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(map);
    let token = TokenDefinition {
        name: map,
        colors: ColorSet::colorless(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        power: None,
        toughness: None,
        keywords: vec![],
        // GAP: Map token's "{1}, {T}, Sacrifice this token: Target
        // creature you control explores." activated ability — no
        // Explore Effect variant; deferred engine work.
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: trig.controller, token }]
}
